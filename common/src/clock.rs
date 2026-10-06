use substreams::errors::Error;
use substreams::pb::sf::substreams::Clock;

#[substreams::handlers::map]
pub fn map_clocks(clock: Clock) -> Result<Clock, Error> {
    Ok(Clock {
        timestamp: clock.timestamp,
        id: clock.id,
        number: clock.number,
    })
}
