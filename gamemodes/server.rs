use omp::{
	core::{
		AddRule,
        DisableInteriorEnterExits, EnableStuntBonusForAll, SetGameModeText,
        SetNameTagsDrawDistance, SetWeather, SetWorldTime, ShowNameTags, ShowPlayerMarkers, UsePedAnims,
    }
};

#[allow(non_snake_case)]
pub fn ServerRule()
{
	SetGameModeText("Harwana Rust");
    ShowPlayerMarkers(0);
    ShowNameTags(true);
    SetNameTagsDrawDistance(40.0);
    EnableStuntBonusForAll(false);
    DisableInteriorEnterExits();
    UsePedAnims();
    SetWeather(2);
    SetWorldTime(11);
	AddRule("developer", "Fanorisky");
}