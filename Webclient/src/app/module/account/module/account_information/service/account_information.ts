import {Injectable} from "@angular/core";
import {APIService} from "../../../../../service/api";
import {AccountInformation} from "../../../domain_value/account_information";
import {NotificationService} from "../../../../../service/notification";
import {Severity} from "../../../../../domain_value/severity";
import {SettingsService} from "../../../../../service/settings";

@Injectable({
    providedIn: "root",
})
export class AccountInformationService {
    private static readonly URL_GET: string = '/account/get';
    private static readonly URL_CREATE_RESEND: string = '/account/create/resend';

    constructor(
        private apiService: APIService,
        private notificationService: NotificationService,
        private settingsService: SettingsService
    ) {
    }

    get(on_success: (AccountInformation) => void): void {
        this.apiService
            .get<AccountInformation>(AccountInformationService.URL_GET, (result) => {
                this.settingsService.set("ACCOUNT_INFORMATION", result);
                on_success.call(on_success, result);
            });
    }

    resend_confirmation(on_response: any): void {
        this.apiService.post(AccountInformationService.URL_CREATE_RESEND, '', () => {
            this.notificationService.propagate(Severity.Info, "serverResponses.mail_confirm");
            on_response.call(on_response);
        }, on_response);
    }
}
