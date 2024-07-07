main.rs
    imports
        use specs
        mod console
        mod componenents
        use components::tilemap
        mod fileio 
        mod systems
        mod resources
        use resources::game_data::GameData
        mod shared
        use shared::{Vector2}
        mod ui
        mod actors
        mod dispatcher_builder
        mod input
        mod event_channel
        mod generators

    main()
        1. create ecs world and register components from 
            a. world
            a. resources
            b. components
            c. ui
            d. event channel
            e. actors
        2. create map
            a. TileMap
        3. intialize ui
        4. create_entity ???
        5. run_game(world)

    run_game( world)
        runs dispatcher_builder
            setup_dispatcher
            turn_dispatcher
            input_dispatcher
            ui_dispatcher
            reunder_dispatcher
        runs setup.dispatch(&world)
        loop
            turn.dispatch
            world.maintain
            wait for ui dispatch
            input.dispatch(&world)
    thoughts:
        status:
            decently commented for our development
        what is happening:
            initializing variables
            running turn structure and map update
        what I want to know returning:
            how does dispatch work,
            what is happening with world.maintain()
        problems:
            dead code
            no overview
            comments rely on dev knowing about ecs
        notes:
            what next?
                dispatcher

dispatcher_builder.rs
    setup_dispatcher
        
general overview
    prowl is a demo of a procedurally generated game. It serves as a proof of concept of our map generation and basic interaction for our game. 

    The concept is a roguelike city builder set in an archipelago. The player controls a ship and navigates the seas between the islands trading for resources.

    The current demo uses perlin noise to randomly generate the islands and map. The player is randomly placed and can navigate the water between islands. There are npc ships, the player must manage resources so as to not run out of food or hp while moving. 

    This is a very stripped down version of the full concept. For future development, the player should be associated with a particular island, and should be able to spend resources to build services and manage development of the civilizaiton on the associated island. 

code overview
    This project is written in rust, runs the game in a terminal instance using a tcod library, and is organized with an entity component system using the specs library for rust. 

    An Entity Component System(ECS) is a common organizational paradigm in game development, but will be unfamiliar to a developer used to an Object Oriented or Tagged Union organization of code. ECS orgization can be boiled down to a core organizational paradigm of all game objects are Entities, Entities have ids, ids are used to associate an id with components that contain functionality. Entities are essentially a grouping of components. Where an OOP system would have increasingly more specific objects as they inherit functionality from their parents, an ECS has increasing object specificity via association between an Entity and existing Components. It's a division between crafting an object as a relation to other similar structures in OOP and crafting an object that is extremely general and then associating with game systems in ECS. 
        In a game development context, an ECS allows for:
            - Flexibility in development. Separation between pieces of functionality so a developer can easily modify function of an entity a more granular level. As a game object usually touches several game systems, being able to modify how a game object interacts with a particular system while retaining other functionality is an advantage.
            - Flexibility in usage. On the fly additions or subtractions of functionality to a Entity by inculding or removing its id within a component's id set allows for changing how a game object interacts with the game during gameplay.
            - Reusability. Game objects are typically created and removed during a gameplay session. ECS offers a system to quickly and easily define a new object's properties upon creation. The same is true for removal, ECS is designed to offer a system to fluidly remove an object from the game.
            - High Performance in a context where many game objects are operating on systems. Keeping track of many independent actors in a context can be very expensive. ECS utilizes an event channel to manage interations and allow for paralell processing of game actions. In addition, ECS manages sets of id's using contiguous memory in the form of hash maps. This means that ECS is capable of leveraging the fast access of the cache to quickly determine which components to access. 
            - Better maintainability in a context of many objects. Games typically contain loads of different actors, objects, and systems. The granular nature of components being asssociated with Entities separates each little function from each other one. Compared to an object oriented organization, it means that a developer or maintainer can find and modify a small file existing in its own context rather than navigating a complex inheritance tree. The mental load that comes from exactly where a function should be contained within the inheritance tree is offloaded, as the correct place to put new functionality is a separate file. The target file to modify a function is separate and named. No need to consider which objects need the functionality in question and how to reach all of them via inheritance. 

    



### 1. **Design Philosophy**

- **ECS:**
  - **Composition Over Inheritance:** Entities are composed of components.
  - **Data-Oriented:** Separates data (components) from behavior (systems).
  - **Modular:** Systems operate on groups of components.

- **Tagged Union:**
  - **Type Safety:** Encapsulates a value that can be one of several types.
  - **Pattern Matching:** Often used with pattern matching to handle different types.
  - **Flexibility:** Allows variables to hold multiple types over time.

- **OOP:**
  - **Inheritance-Based:** Uses class hierarchies to share and extend behavior.
  - **Encapsulation:** Data and behavior are bundled within objects.
  - **Polymorphism:** Objects can be treated as instances of their base class.

### 2. **Flexibility**

- **ECS:**
  - **Highly Flexible:** Easily add, remove, or modify components.
  - **Reusability:** Components and systems are highly reusable across different entities.

- **Tagged Union:**
  - **Moderately Flexible:** Allows for multiple types but requires careful handling.
  - **Type-Safe Flexibility:** Each variant is explicitly handled, reducing errors.

- **OOP:**
  - **Less Flexible:** Changing class hierarchies can be complex and error-prone.
  - **Inheritance Bound:** Adding new behaviors can require extensive refactoring.

### 3. **Performance**

- **ECS:**
  - **Cache-Friendly:** Components are stored contiguously, enhancing cache performance.
  - **Parallelism:** Systems can process multiple entities in parallel.

- **Tagged Union:**
  - **Performance Varies:** Depends on implementation and usage patterns.
  - **Memory Overhead:** Can have additional memory overhead due to type tagging.

- **OOP:**
  - **Cache Misses:** Objects scattered in memory can lead to cache misses.
  - **Single-Threaded:** Typically single-threaded, although can be multi-threaded with effort.

### 4. **Maintainability**

- **ECS:**
  - **Separation of Concerns:** Clear separation between data and logic.
  - **Modular:** Easier to maintain and extend due to decoupled components and systems.

- **Tagged Union:**
  - **Pattern-Based:** Clear and maintainable when using pattern matching.
  - **Explicit Handling:** Each type case is explicitly handled, reducing hidden errors.

- **OOP:**
  - **Complex Hierarchies:** Can become difficult to maintain with deep inheritance trees.
  - **Coupling:** Tight coupling of data and behavior can lead to maintenance challenges.

### Summary

- **ECS** is ideal for large-scale systems with numerous interacting entities, offering flexibility, performance, and maintainability through composition and separation of data and logic.
- **Tagged Unions** provide type safety and flexibility for handling multiple types within a single variable, often used in functional programming languages.
- **OOP** is well-suited for systems where encapsulation and polymorphism are important, but can become less flexible and harder to maintain as complexity grows. 

Choosing between these paradigms depends on the specific requirements and constraints of your project.