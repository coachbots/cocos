mod api;
mod position;
mod peripheral;
pub mod master;

trait HandlesTick1Ms { fn on_tick1(&mut self); }
trait HandlesTick10Ms { fn on_tick10(&mut self); }
trait HandlesTick100Ms { fn on_tick100(&mut self); }
trait HandlesTick1000Ms { fn on_tick1000(&mut self); }
