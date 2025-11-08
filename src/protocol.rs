//! Definition of the core `Protocol` trait to sansio
//!
//! The [`Protocol`] trait provides the necessary abstractions for defining
//! sans-io protocol. It is simple but powerful and is used as the foundation
//! for the rest of sansio library.

use std::time::Instant;

/// The `Protocol` trait is a simplified interface making it easy to write
/// network protocols in a modular and reusable way, decoupled from the
/// underlying network and timer, etc. It is one of sans-io fundamental abstractions.
pub trait Protocol<Rin, Win, Ein> {
    /// Associated output read type
    type Rout;
    /// Associated output write type
    type Wout;
    /// Associated output event type
    type Eout;
    /// Associated result error type
    type Error;

    /// Handles Rin and returns Rout for next inbound handler handling
    fn handle_read(&mut self, msg: Rin) -> Result<(), Self::Error>;

    /// Polls Rout from internal queue for next inbound handler handling
    fn poll_read(&mut self) -> Option<Self::Rout>;

    /// Handles Win and returns Wout for next outbound handler handling
    fn handle_write(&mut self, msg: Win) -> Result<(), Self::Error>;

    /// Polls Wout from internal queue for next outbound handler handling
    fn poll_write(&mut self) -> Option<Self::Wout>;

    /// Handles event
    fn handle_event(&mut self, _evt: Ein) -> Result<(), Self::Error> {
        Ok(())
    }

    /// Polls event
    fn poll_event(&mut self) -> Option<Self::Eout> {
        None
    }

    /// Handles timeout
    fn handle_timeout(&mut self, _now: Instant) -> Result<(), Self::Error> {
        Ok(())
    }

    /// Polls timeout
    fn poll_timeout(&mut self) -> Option<Instant> {
        None
    }

    /// Closes protocol
    fn close(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl<P, Rin, Win, Ein> Protocol<Rin, Win, Ein> for &mut P
where
    P: Protocol<Rin, Win, Ein> + ?Sized,
{
    type Rout = P::Rout;
    type Wout = P::Wout;
    type Eout = P::Eout;
    type Error = P::Error;

    fn handle_read(&mut self, msg: Rin) -> Result<(), P::Error> {
        (**self).handle_read(msg)
    }

    fn poll_read(&mut self) -> Option<P::Rout> {
        (**self).poll_read()
    }

    fn handle_write(&mut self, msg: Win) -> Result<(), P::Error> {
        (**self).handle_write(msg)
    }

    fn poll_write(&mut self) -> Option<P::Wout> {
        (**self).poll_write()
    }

    fn handle_event(&mut self, evt: Ein) -> Result<(), P::Error> {
        (**self).handle_event(evt)
    }

    fn poll_event(&mut self) -> Option<P::Eout> {
        (**self).poll_event()
    }

    fn handle_timeout(&mut self, now: Instant) -> Result<(), P::Error> {
        (**self).handle_timeout(now)
    }

    fn poll_timeout(&mut self) -> Option<Instant> {
        (**self).poll_timeout()
    }

    fn close(&mut self) -> Result<(), P::Error> {
        (**self).close()
    }
}

impl<P, Rin, Win, Ein> Protocol<Rin, Win, Ein> for Box<P>
where
    P: Protocol<Rin, Win, Ein> + ?Sized,
{
    type Rout = P::Rout;
    type Wout = P::Wout;
    type Eout = P::Eout;
    type Error = P::Error;

    fn handle_read(&mut self, msg: Rin) -> Result<(), P::Error> {
        (**self).handle_read(msg)
    }

    fn poll_read(&mut self) -> Option<P::Rout> {
        (**self).poll_read()
    }

    fn handle_write(&mut self, msg: Win) -> Result<(), P::Error> {
        (**self).handle_write(msg)
    }

    fn poll_write(&mut self) -> Option<P::Wout> {
        (**self).poll_write()
    }

    fn handle_event(&mut self, evt: Ein) -> Result<(), P::Error> {
        (**self).handle_event(evt)
    }

    fn poll_event(&mut self) -> Option<P::Eout> {
        (**self).poll_event()
    }

    fn handle_timeout(&mut self, now: Instant) -> Result<(), P::Error> {
        (**self).handle_timeout(now)
    }

    fn poll_timeout(&mut self) -> Option<Instant> {
        (**self).poll_timeout()
    }

    fn close(&mut self) -> Result<(), P::Error> {
        (**self).close()
    }
}
