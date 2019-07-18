use specs::{
    world::Builder, Component, Read, ReadStorage, System, SystemData, VecStorage, World,
    WriteStorage,
};

#[derive(Debug)]
struct Pair {
    i: u64,      //id
    n: String, //name
}
impl Component for Pair{
    type Storage = VecStorage<Self>;
}

#[derive(Debug)]
struct Edge {
    id: u64,    //id of edge,
    dist: u32,  //weight of edge
    start: Pair, //start pos of directed movement
    end: Pair,
}
impl Component for Edge{
    type Storage = VecStorage<Self>;
}

#[derive(Debug)]
struct Island {
    loc: Pair,            //name and id of island
    pop: u64,            //population of island, affected by events
    max_pop: u64,        //max pop is compared against pop for events, affected by events
    targs: Vec<u64>,     //targs list of id's to set edges to
    adj_list: Vec<Edge>, //edges are set after creation
    res: Vec<String>,    //strings containing resource names, pobs good for enums
}
impl Component for Island{
    type Storage = VecStorage<Self>;
}

fn assign_edges(islands: &[Island]) {
    let mut j: u64 = 0; //id for each edge, i think it's gonna be nice for record keeping
    for i in islands {
        for targ in i.targs {
            i.adj_list.push(Edge {
                id: j,
                dist: 50,
                start: Pair{ i: i.loc.id,  name:i.loc.name.to_string()},
                //keep an eye on end, only works because of the way island ids are genereated
                end: Pos{id: islands[targ as usize].loc.id, name: islands[targ as usize].loc.name.to_string()},
            });
            j += 1;
        }
    }
}
fn main() {
    let mut world = World::new();
    world.add_enitity(Island);
    world.add_entity(Edge);
    world.add_entity(Pair);

    let mut dispatcher = specs::DispatcherBuilder::new()
        .with()
        .with()
    
    dispatcher.dispatch(&mut world.res):
    world.maintain();
}
fn build(){
    let isoles = vec![
        "Albion".to_string(),
        "Dreck".to_string(),
        "Neboa".to_string(),
        "Wenden".to_string(),
    ];
    let mut locs = Vec::new();
    let mut targ_list = Vec::new();
    let mut islands = Vec::new();
    let mut j: u64 = 0; //note, fix later, figure out map
    for i in &isoles {
        locs.push(Pos {
            id: j,
            name: i.to_string(),
        });
        targ_list.push(j);
        j += 1;
    }
    locs.reverse();
    while !locs.is_empty() {
        islands.push(Island {
            loc: locs.pop().unwrap(),
            pop: 50,
            max_pop: 75,
            targs: targ_list.clone(),
            adj_list: Vec::new(),
            res: vec!["iron".to_string(), "gold".to_string()],
        });
    }
    assign_edges(&islands);
    println!("{:?}", islands);
    */
}
