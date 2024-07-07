## Design Overview
### general overview
    prowl is a demo of a procedurally generated game. It serves as a proof of concept of our map generation and basic interaction for our game. 

    The concept is a roguelike city builder set in an archipelago. The player controls a ship and navigates the seas between the islands trading for resources.

    The current demo uses perlin noise to randomly generate the islands and map. The player is randomly placed and can navigate the water between islands. There are npc ships, the player must manage resources so as to not run out of food or hp while moving. 

    This is a very stripped down version of the full concept. For future development, the player should be associated with a particular island, and should be able to spend resources to build services and manage development of the civilizaiton on the associated island. 

### Design Decisions
    This project is written in rust, runs the game in a terminal instance using a tcod library, and is organized with an entity component system using the specs library for rust. 

#### Roguelike
    This game is a roguelike, and is meant to be replayable and to have each instance unique. We use procedural generation to make unique maps with unique play environments.

#### Why an ASCII Based Terminal Display
    This project emulates the game rogue which uses an ascii graphical interface. As an homage, we chose to emulate that design decision.

#### Why ECS?

    An Entity Component System(ECS) is a common organizational paradigm in game development because it allows for felxibility between game objects and effective use of memory resources in a complex environment. ECS might be unfamiliar to a developer used to an Object Oriented or Tagged Union organization of code so as a guide here is a short definition of it:
        ECS orgization can be boiled down to a core organizational paradigm of all game objects are Entities, Entities have ids, ids are used to associate an id with components that contain functionality. Entities are essentially a grouping of components. Where an OOP system would have increasingly more specific objects as they inherit functionality from their parents, an ECS has increasing object specificity via association between an Entity and existing Components. It's a division between crafting an object as a relation to other similar structures in OOP and crafting an object that is extremely general and then associating with game systems in ECS. 

        In a game development context, an ECS allows for:
            - Flexibility in development. Separation between pieces of functionality so a developer can easily modify function of an entity a more granular level. As a game object usually touches several game systems, being able to modify how a game object interacts with a particular system while retaining other functionality is an advantage.
            - Flexibility in usage. On the fly additions or subtractions of functionality to a Entity by inculding or removing its id within a component's id set allows for changing how a game object interacts with the game during gameplay.
            - Reusability. Game objects are typically created and removed during a gameplay session. ECS offers a system to quickly and easily define a new object's properties upon creation. The same is true for removal, ECS is designed to offer a system to fluidly remove an object from the game.
            - High Performance in a context where many game objects are operating on systems. Keeping track of many independent actors in a context can be very expensive. ECS utilizes an event channel to manage interations and allow for paralell processing of game actions. In addition, ECS manages sets of id's using contiguous memory in the form of hash maps. This means that ECS is capable of leveraging the fast access of the cache to quickly determine which components to access. 
            - Better maintainability in a context of many objects. Games typically contain loads of different actors, objects, and systems. The granular nature of components being asssociated with Entities separates each little function from each other one. Compared to an object oriented organization, it means that a developer or maintainer can find and modify a small file existing in its own context rather than navigating a complex inheritance tree. The mental load that comes from exactly where a function should be contained within the inheritance tree is offloaded, as the correct place to put new functionality is a separate file. The target file to modify a function is separate and named. No need to consider which objects need the functionality in question and how to reach all of them via inheritance. 

#### Why TCOD
    TCOD is commonly used for terminal based ascii graphical representations. It also was in the most complete state of development within the Rust environment at the time of initial development. 