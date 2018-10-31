/* vi: set ts=4 expandtab shiftwidth=4: */

// Copyright 2018 by the Digital Aggregates Corporation
// Licensed under the terms in LICENSE.txt
// Author: Chip Overclock
// mailto:coverclock@diag.com
// https://github.com/coverclock/com-diag-rustler

/// This is an interface that describes the API for any implementation of a
/// Throttle. Throttles are mechanisms that shape event emission rates or
/// police event admission rates. Frequently, throttles are implemented using
/// a virtual scheduler or a leaky bucket.
///
/// This module is based on the Go implementation in the Vamoose repository.
///
pub mod throttle {

    use std::string;
    use std::marker;
    use ticks::ticks;
 
    /// Events is the type used to indicate how many events have been emitted since
    /// the last update of the throttle. An event can be the emission of a single
    /// packet, or a single byte, or a single bit, etc. It is up to the application
    /// to define what an event is. The throttle is defined in terms of ticks per
    /// event.
    pub type Events = i64;
    
    /// Throttle defines the standard API for rate limiting implementations.
    pub trait Throttle : marker::Sync + marker::Send {

        /***************************************************************************
         * INSPECTORS
         **************************************************************************/
       
        fn size_of(& self) -> usize;

        /***************************************************************************
         * CONVERTORS
         **************************************************************************/
         
        /// as_string returns the underlying object as a printable string. This
        /// is equivalent to to_string but can be called from the trait without
        /// having to know the actual super type or size_of.
        fn as_string(& self) -> string::String;

        /***************************************************************************
         * SETTERS
         **************************************************************************/
    
        /// reset a throttle back to its initial state. This is used during construction,
        /// but can also be used by an application when a calamitous happenstance
        /// occurs, like the far end disconnecting and reconnecting.
        fn reset(& mut self, now: ticks::Ticks);
        
        /***************************************************************************
         * MUTATORS
         **************************************************************************/
    
        /// request computes, given the current time in ticks, how long of a delay
        /// in ticks would be necessary before the next event were emitted for that
        /// emission to be in compliance with the traffic contract.
        fn request(& mut self, now: ticks::Ticks) -> ticks::Ticks;
        
        /// commits updates the throttle with the number of events having been emitted
        /// starting at the time specified in the previous Request, and returns false
        /// if the throttle is alarmed, indicating the application might want to slow it
        /// down a bit, true otherwise.
        fn commits(& mut self, events: Events) -> bool;

        /// commit is equivalent to calling Commits with one event.
        fn commit(& mut self) -> bool;
        
        /// admits combines calling Request with the current time in ticks with
        /// calling and returning the value of Commits with the number of events.
        fn admits(& mut self, now: ticks::Ticks, events: Events) -> bool;
        
        /// admit is equivalent to calling Admits with one event.
        fn admit(& mut self, now: ticks::Ticks) -> bool;
        
        /// update is equivalent to calling Admits with zero events. It is a way to
        /// update the throttle with the current time, with no event emission. This
        /// marks the passage of time during which the emission stream is idle, which
        /// may bring the throttle back into compliance with the traffic contract (and
        /// will do so if time has advanced at least as much as the value returned by
        /// get_expected).
        fn update(& mut self, now: ticks::Ticks) -> bool;

        /***************************************************************************
         * GETTERS
         **************************************************************************/
        
        /// get_expected returns the number of ticks that would be necessary for the
        /// caller to delay for the event stream  to comply to the traffic contract with
        /// no limit penalty accumulated given the current state of the throttle. For
        /// throttles whose implementations differ from that of the Generic Cell Rate
        /// Algorithm, the value returned may be the same as that returned by Request
        /// given the current state of the throttle, or some other value entirely.
        fn get_expected(& self) -> ticks::Ticks;
        
        /// is_empty returns true if the throttle is empty, that is, it has no accumulated
        /// deficit ticks.
        fn is_empty(& self) -> bool;
        
        /// is_full returns true if the throttle is full, that is, its accumulated deficit
        /// ticks is greater than or equal to its limit.
        fn is_full(& self) -> bool;
        
        /// is_alarmed returns true if the throttle is alarmed, that is, its accumulated
        /// deficit ticks is greater than its limit, indicating that the event
        /// emission stream is out of compliance with the traffic contract.
        fn is_alarmed(& self) -> bool;

        /***************************************************************************
         * SENSORS
         **************************************************************************/
    
        /// emptied returns true if the throttle just emptied in the last action.
        fn emptied(& self) -> bool;
        
        /// filled returns true if the throttle just filled in the last action.
        fn filled(& self) -> bool;
        
        /// alarmed returns true if the throttle just alarmed in the last action.
        fn alarmed(& self) -> bool;
        
        /// cleared returns true if the throttle just unalarmed in the last action,
        /// indicating that the event emission stream has returned to being
        /// compliant with the traffic contract.
        fn cleared(& self) -> bool;
   
    }

}
