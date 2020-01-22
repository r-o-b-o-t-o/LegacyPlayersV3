use mysql_connection::tools::Select;

use crate::dto::Failure;
use crate::modules::armory::Armory;
use crate::modules::armory::domain_value::CharacterInfo;
use crate::modules::armory::dto::CharacterInfoDto;
use crate::modules::armory::tools::GetCharacterGear;

pub trait GetCharacterInfo {
  fn get_character_info(&self, character_info_id: u32) -> Result<CharacterInfo, Failure>;
  fn get_character_info_by_value(&self, character_info: CharacterInfoDto) -> Result<CharacterInfo, Failure>;
}

impl GetCharacterInfo for Armory {
  fn get_character_info(&self, character_info_id: u32) -> Result<CharacterInfo, Failure> {
    let params = params!(
      "id" => character_info_id
    );
    self.db_main.select_wparams_value("SELECT * FROM armory_character_info WHERE id=:id", &|mut row| {
      Ok(CharacterInfo {
        id: row.take(0).unwrap(),
        gear: self.get_character_gear(row.take(1).unwrap()).unwrap(),
        hero_class_id: row.take(2).unwrap(),
        level: row.take(3).unwrap(),
        gender: row.take(4).unwrap(),
        profession1: row.take_opt(5).unwrap().ok(),
        profession2: row.take_opt(6).unwrap().ok(),
        talent_specialization: row.take_opt(7).unwrap().ok(),
        race_id: row.take(8).unwrap()
      })
    }, params).unwrap_or_else(|| Err(Failure::Unknown))
  }

  fn get_character_info_by_value(&self, character_info: CharacterInfoDto) -> Result<CharacterInfo, Failure> {
    let character_gear_res = self.get_character_gear_by_value(character_info.gear.clone());
    if character_gear_res.is_err() {
      return Err(character_gear_res.err().unwrap());
    }

    let params = params!(
      "gear_id" => character_gear_res.unwrap().id,
      "hero_class_id" => character_info.hero_class_id,
      "level" => character_info.level,
      "gender" => character_info.gender,
      "profession1" => character_info.profession1.clone(),
      "profession2" => character_info.profession2.clone(),
      "talent_specialization" => character_info.talent_specialization.clone(),
      "race_id" => character_info.race_id
    );
    self.db_main.select_wparams_value("SELECT * FROM armory_character_info WHERE gear_id=:gear_id \
      AND hero_class_id=:hero_class_id AND level=:level AND gender=:gender \
      AND ((ISNULL(:profession1) AND ISNULL(profession1)) OR profession1 = :profession1) \
      AND ((ISNULL(:profession2) AND ISNULL(profession2)) OR profession2 = :profession2) \
      AND ((ISNULL(:talent_specialization) AND ISNULL(talent_specialization)) OR talent_specialization = :talent_specialization) \
      AND race_id=:race_id", &|mut row| {
      Ok(CharacterInfo {
        id: row.take(0).unwrap(),
        gear: self.get_character_gear(row.take(1).unwrap()).unwrap(),
        hero_class_id: row.take(2).unwrap(),
        level: row.take(3).unwrap(),
        gender: row.take(4).unwrap(),
        profession1: row.take_opt(5).unwrap().ok(),
        profession2: row.take_opt(6).unwrap().ok(),
        talent_specialization: row.take_opt(7).unwrap().ok(),
        race_id: row.take(8).unwrap()
      })
    }, params).unwrap_or_else(|| Err(Failure::Unknown))
  }
}