use uom::si::f32::Ratio;
use uom::si::f32::Time;
use uom::si::ratio::percent;
use uom::si::time::hour;
use uom::si::time::minute;

pub(crate) fn get_raw_percent(percentage: Ratio) -> f32 {
    percentage.round::<percent>().get::<percent>()
}

pub(crate) fn get_raw_hours(time: Time) -> f32 {
    time.floor::<hour>().get::<hour>()
}

pub(crate) fn get_raw_minutes(time: Time) -> f32 {
    time.fract::<hour>().floor::<minute>().get::<minute>()
}
