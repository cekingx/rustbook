use averaged_collection::AveragedCollection;

fn main() {
    let mut collection = AveragedCollection::new();
    collection.add(1);
    collection.add(2);
    println!("the average: {}", collection.average());
}
