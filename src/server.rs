use omp::{
	core::{
		AddRule,
        DisableInteriorEnterExits, EnableStuntBonusForAll, SetGameModeText,
        SetNameTagsDrawDistance, SetWeather, SetWorldTime, ShowNameTags, ShowPlayerMarkers,
    }
};

#[allow(non_snake_case)]
pub fn ServerRule()
{
	SetGameModeText("Grand Larceny");
    ShowPlayerMarkers(1);
    ShowNameTags(true);
    SetNameTagsDrawDistance(40.0);
    EnableStuntBonusForAll(false);
    DisableInteriorEnterExits();
    SetWeather(2);
    SetWorldTime(11);
	AddRule("developer", "Fanorisky");
}