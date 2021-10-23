    pub trait Observer {
        fn run(&self, data: u32) -> ();        
    }

    pub trait Observable {
        fn register(&mut self, o: dyn Observer) -> ();
        fn notify() -> ();
    }

    pub struct Entity{
       name: String,
       count: u32,
    }
    pub struct Station {
        observers: Vec<dyn Observer>
    }

    impl Observer for Entity {
        fn run(&self, data: u32) -> () {
            println!("{} got Message: {}", self.name, data);
        }
    }

    impl Observable for Station {
        fn register(&mut self, o: dyn Observer) -> () {
            self.observers.push(o);
        }
        fn notify(&self) -> () {
            for observer in self.observers.iter() {
                observer.run(self.observers.len);
            }
        }

    }
#[cfg(test)]
mod test {
   use super::Entity;
   use super::Station;
   #[test]
   fn basics() {

   let mut station = Station::new();

   let e1 = Entity::new("entity 1", 0);
   let e2 = Entity::new("entity 2", 0);

   assert_eq!(e1.count, 0);
   e1.run(2);
   assert_eq!(e1.count, 1);
   station.notify();
   assert_eq!(e1.count, 1);
   assert_eq!(e2.count, 0);

   station.register(e1);
   station.notify();
   assert_eq!(e1.count, 2);

   station.register(e2);
   station.notify();
   assert_eq!(e1.count, 3);
   assert_eq!(e2.count, 1);
   }
}
