use std::cell::Cell;
use std::cell::RefCell;
use std::rc::Rc;

pub fn run() {
    // Box<T>
    let boxed = Box::new(10);
    assert_eq!(*boxed, 10);

    let my_box = Box::new(5);
    let an_integer: i32 = *my_box;
    println!("{:?}", my_box);
    println!("{:?}", an_integer);

    // Call<T>
    let container = Cell::new(10);
    let ten = container.get();
    {
        container.set(12);
    }
    let twelve = container.get();
    assert_eq!(ten, 10);
    assert_eq!(twelve, 12);

    // RefCell<T>
    let container = RefCell::new(10);
    {
        let _c = container.borrow();
        // You may borrow as immutable as many times as you want,...
        assert!(container.try_borrow().is_ok());
        // ...but cannot borrow as mutable because
        // it is already borrowed as immutable.
        assert!(container.try_borrow_mut().is_err());
    }
    // After the first borrow as mutable...
    let _c = container.borrow_mut();
    // ...you cannot borrow in any way.
    assert!(container.try_borrow().is_err());
    assert!(container.try_borrow_mut().is_err());

    // Rc<T>
    let mut c = Rc::new(11);

    {
        // After borrowing as immutable...
        let _first = c.clone();

        // ...you can no longer borrow as mutable,...
        assert_eq!(Rc::get_mut(&mut c), None);

        // ...but can still borrow as immutable.
        let _second = c.clone();

        let _third = _second.clone();

        let _fourth = _third.clone();

        // Here we have 4 pointer ("c", "_first", "_second", "_third", "_fourth").
        assert_eq!(Rc::strong_count(&c), 5);
    }

    // After we drop the last two, we remain only with "c" itself.
    assert_eq!(Rc::strong_count(&c), 1);

    // And now we can borrow it as mutable.
    let y = Rc::get_mut(&mut c).unwrap();
    *y += 1;

    assert_eq!(*c, 12);

    let z = Rc::get_mut(&mut c).unwrap();
    *z += 3;

    assert_eq!(*c, 15);
}
