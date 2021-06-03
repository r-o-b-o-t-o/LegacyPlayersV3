import {Component, OnInit} from "@angular/core";
import {Preset} from "../../domain_value/preset";
import {NotificationService} from "../../../../../../../../service/notification";
import {Severity} from "../../../../../../../../domain_value/severity";
import {SettingsService} from "../../../../../../../../service/settings";

@Component({
    selector: "RaidBrowser",
    templateUrl: "./raid_browser.html",
    styleUrls: ["./raid_browser.scss"]
})
export class RaidBrowserComponent implements OnInit {

    presets: Array<Preset> = [];

    show_remove_widget: boolean = false;
    show_add_widget: boolean = false;

    constructor(
        private notificationService: NotificationService,
        private settingsService: SettingsService
    ) {
    }

    ngOnInit(): void {
        if (this.settingsService.check("viewer_presets"))
            this.presets = this.settingsService.get("viewer_presets");
    }

    toggleRemoveWidget(): void {
        this.show_add_widget = false;
        this.show_remove_widget = !this.show_remove_widget;
    }

    toggleAddWidget(): void {
        this.show_remove_widget = false;
        this.show_add_widget = !this.show_add_widget;
    }

    close_remove_widget(): void {
        this.show_remove_widget = false;
    }

    close_add_widget(): void {
        this.show_add_widget = false;
    }

    remove_preset(preset_name: string): void {
        this.presets = this.presets.filter(preset => preset.name !== preset_name);
        this.settingsService.set("viewer_presets", this.presets);
        this.notificationService.propagate(Severity.Success, "Preset has been removed!");
    }

    add_preset(preset: Preset): void {
        const current_presets = this.presets.filter(i_preset => i_preset.name !== preset.name);
        current_presets.push(preset);
        this.presets = current_presets;
        this.settingsService.set("viewer_presets", this.presets);
        this.notificationService.propagate(Severity.Success, "Preset has been added!");
    }
}
