#![allow(dead_code)]
#![allow(patterns_in_fns_without_body)]

/*** Part 1 **********************************/
pub fn capitalize_first(input: &str) -> String {
    let mut c = input.chars();
    //TODO: use c.next() and match on it.
    // documentation for Chars: https://doc.rust-lang.org/std/str/struct.Chars.html?search= 
    // NOTE:Chars implement the Iterator trait.
    unimplemented!();
}

#[test]
fn test_success() {
    assert_eq!(capitalize_first("hello"), "Hello");
}

#[test]
fn test_empty() {
    assert_eq!(capitalize_first(""), "");
}


/*** Part 2 **********************************/

struct ProductEventData {
    quantity: usize,
    timestamp: usize,
    id: String,
}

/// That represent each of the possible types of events that can occur to a product, and.
/// We can unify all product events as a single type.
enum ProductEvent {
    Reserved(ProductEventData),
    Released(ProductEventData),
    Shipped(ProductEventData),
}

trait Aggregate {
    type Item;
    fn version(&self) -> usize;
    fn apply(mut self, evt: &Self::Item) -> Self where Self: Sized;
}

impl ProductEvent {
    fn reserved(id: &str, qty: usize) -> Self {
        Self::Reserved(ProductEventData {
            quantity: qty,
            id: id.to_owned(),
            timestamp: 1,
        })
    }

    fn shipped(id: &str, qty: usize) -> ProductEvent {
        ProductEvent::Shipped(ProductEventData {
            quantity: qty,
            id: id.to_owned(),
            timestamp: 1,
        })
    }
    fn released(id: &str, qty: usize) -> ProductEvent {
        ProductEvent::Released(ProductEventData {
            quantity: qty,
            id: id.to_owned(),
            timestamp: 1,
        })
    }
}

/// Aggregates are calculations for a single entity, not for the entire state of your application.
/// Further, aggregates are short-lived. They live long enough to calculate state and validate a command, and that’s it.
/// If you’re building a stateless service, it’s going to dispose of the aggregate at the end of the request.
#[derive(Debug, PartialEq)]
struct ProductAggregate {
    version: usize,
    qty_on_hand: usize,
    id: String,
}

impl ProductAggregate {
    fn new(id: &str, initial_quantity: usize) -> Self {
        Self {
            version: 1,
            qty_on_hand: initial_quantity,
            id: id.to_owned(),
        }
    }

    fn reserve_quantity(&self, qty: usize) -> Result<Vec<ProductEvent>, String> {
        if qty > self.qty_on_hand {
            let msg = format!(
                "Cannot reserve more than on hand quantity ({})",
                self.qty_on_hand
            );
            Err(msg)
        } else if self.version == 0 {
            Err(
                "Cannot apply a command to an un-initialized aggregate. Did you forget something?"
                    .to_owned(),
            )
        } else {
            Ok(vec![ProductEvent::reserved(&self.id, qty)])
        }
    }

    fn release_quantity(&self, qty: usize) -> Result<Vec<ProductEvent>, String> {
        Ok(vec![ProductEvent::released(&self.id, qty)])
    }

    fn ship_quantity(&self, qty: usize) -> Result<Vec<ProductEvent>, String> {
        Ok(vec![ProductEvent::shipped(&self.id, qty)])
    }

    fn quantity(&self) -> usize {
        self.qty_on_hand
    }
}

impl Aggregate for ProductAggregate {
    type Item = ProductEvent;

    fn version(&self) -> usize {
        self.version
    }

    /// The result of applying one event to an aggregate produces an aggregate with a new state.
    fn apply(mut self, _evt: &ProductEvent) -> ProductAggregate {
        //TODO:
        //* inc the version
        // * pattern match on evt and feed 
        unimplemented!();
    }
}



#[test]
fn cqrs_should_work() {
    let agg = ProductAggregate::new("SOAP", 100);

    let agg2 = agg.apply(&ProductEvent::reserved("SOAP", 10));
    assert_eq!(ProductAggregate { version: 2, qty_on_hand: 90, id: "SOAP".to_string() }, agg2);

    let agg3 = agg2.apply(&ProductEvent::reserved("SOAP", 30));
    assert_eq!(ProductAggregate { version: 3, qty_on_hand: 60, id: "SOAP".to_string() }, agg3);
}