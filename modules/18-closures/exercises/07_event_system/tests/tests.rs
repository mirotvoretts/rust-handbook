use ex_18_07_event_system::EventBus;
use std::cell::RefCell;
use std::rc::Rc;

#[test]
fn broadcasts_in_registration_order() {
    let log: Rc<RefCell<Vec<String>>> = Rc::new(RefCell::new(Vec::new()));

    let mut bus = EventBus::new();

    let l1 = Rc::clone(&log);
    bus.subscribe(move |msg| l1.borrow_mut().push(format!("a:{msg}")));
    let l2 = Rc::clone(&log);
    bus.subscribe(move |msg| l2.borrow_mut().push(format!("b:{msg}")));

    assert_eq!(bus.subscriber_count(), 2);

    bus.publish("hi");
    assert_eq!(&*log.borrow(), &["a:hi", "b:hi"]);
}

#[test]
fn subscriber_keeps_mutable_state() {
    let count = Rc::new(RefCell::new(0));

    let mut bus = EventBus::new();
    let c = Rc::clone(&count);
    bus.subscribe(move |_| *c.borrow_mut() += 1);

    bus.publish("x");
    bus.publish("y");
    bus.publish("z");

    assert_eq!(*count.borrow(), 3);
}

#[test]
fn empty_bus_publishes_nothing() {
    let mut bus = EventBus::new();
    assert_eq!(bus.subscriber_count(), 0);
    bus.publish("nobody listens"); // не должно паниковать
}
