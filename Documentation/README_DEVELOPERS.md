# For Developers

## Overview
prowl is a demo of a procedurally generated game. It serves as a proof of concept of our map generation and basic interaction for our game. 

The concept is a roguelike city builder set in an archipelago. The player controls a ship and navigates the seas between the islands trading for resources.

The current demo uses perlin noise to randomly generate the islands and map. The player is randomly placed and can navigate the water between islands. There are npc ships, the player must manage resources so as to not run out of food or hp while moving. 

This is a very stripped down version of the full concept. For future development, the player should be associated with a particular island, and should be able to spend resources to build services and manage development of the civilizaiton on the associated island. 

## Design Overview
This project is written in rust, runs the game in a terminal instance using a tcod library, and is organized with an entity component system using the specs library for rust. 

An Entity Component System(ECS) is a common organizational paradigm in game development, but will be unfamiliar to a developer used to an Object Oriented or Tagged Union organization of code. ECS orgization can be boiled down to a core organizational paradigm of all game objects are Entities, Entities have ids, ids are used to associate an id with components that contain functionality. Entities are essentially a grouping of components. Where an OOP system would have increasingly more specific objects as they inherit functionality from their parents, an ECS has increasing object specificity via association between an Entity and existing Components. It's a division between crafting an object as a relation to other similar structures in OOP and crafting an object that is extremely general and then associating with game systems in ECS. 
    In a game development context, an ECS allows for:
        
- Flexibility in development. Separation between pieces of functionality so a developer can easily modify function of an entity a more granular level. As a game object usually touches several game systems, being able to modify how a game object interacts with a particular system while retaining other functionality is an advantage.
        
- Flexibility in usage. On the fly additions or subtractions of functionality to a Entity by inculding or removing its id within a component's id set allows for changing how a game object interacts with the game during gameplay.
        
- Reusability. Game objects are typically created and removed during a gameplay session. ECS offers a system to quickly and easily define a new object's properties upon creation. The same is true for removal, ECS is designed to offer a system to fluidly remove an object from the game.
        
- High Performance in a context where many game objects are operating on systems. Keeping track of many independent actors in a context can be very expensive. ECS utilizes an event channel to manage interations and allow for paralell processing of game actions. In addition, ECS manages sets of id's using contiguous memory in the form of hash maps. This means that ECS is capable of leveraging the fast access of the cache to quickly determine which components to access. 
        
- Better maintainability in a context of many objects. Games typically contain loads of different actors, objects, and systems. The granular nature of components being asssociated with Entities separates each little function from each other one. Compared to an object oriented organization, it means that a developer or maintainer can find and modify a small file existing in its own context rather than navigating a complex inheritance tree. The mental load that comes from exactly where a function should be contained within the inheritance tree is offloaded, as the correct place to put new functionality is a separate file. The target file to modify a function is separate and named. No need to consider which objects need the functionality in question and how to reach all of them via inheritance. 


## Development Goals 

As of 7/6/25, the program:
    
- Runs( Hooray! Well, at least on linux)
    
- Creates a procedurally generated Map
    
- Creates various game objects
        
- The player
        
- Ai controlled Npcs
        
- Islands which can be interacted with
    
- Assigns a set of game resources to the game objects that are interacted with by game systems.
        Currently:
            
- HP
            
- Food
            
- Money
    
- Has systems that utilizes these resouces to manipulate the game.
    
- Allows for player movement and user initiated interaction.
    
- Has an endpoint, the player runs out of hp and dies.

Taking these points as existing functionality, further development should focus on expanding or creating the following systems. Many of these concepts have been partially implemented and already are attatched to the game actors.
    Overview of development goals:
        The player and their home island should seek to achieve victory over the other islands in the archipelago. What victory entails is not yet determined. Through several branching gameplans of accruing economic might, martial prowess, or diplomatic strength, the player should seek to build an advantage over the npcs that are also attempting to achieve their similar goals through one of the delineated paths. 
        As this game seeks to be procedurally generated and replayable, these systems and goals should be generated during game creation. 
    Granular Development points:
        1. Combat
            
- Player should be able to engage in combat with npc ships.
            
- This should use the Weapon Component to modify hp damage done.
            
- There should be a reward garnered from defeating an npc ship. 
        2. Economy
            
- Currently the Player can buy food in exchange for money.
            
- Implement selling resources at islands.
            
- Implement the purchase of the remaining resources, Wood and Metal, and the weapon components from the islands.
        3. Player should be associated with an island and be able to garner resources when docking with home island.
            
- Create an island development system where money, wood, and metal can be spent to modify the home island to increase the amount of resources the player receives from docking with their home island.
        4. Player functions.
            
- A level up system where permanent upgrades can be applied to the player.
            
- An inventory system where more than 1 item may be stored, for example a ship should be able to hold more than 1 weapon and switch between them.
        5. Actor systems.
            
- Ships should be able to permanently improve
            
- Islands should be able to improve or decline
        6. Ai systems
            The npc ships are currently generated only once and slowly run out of hp and die. This is fine for a proof of concept, but is a poor gameloop for an actual game.
            AI Ships
                
- AI controlled ships(AICS) should be generated as the game progresses from non
-player associated islands
                
- AICS should have a power level associated with them based on their strength.
                
- AICS should seek out combat that makes sense for their power level.
                
- AICS should have characteristics determined by the Island that they spawned from.
                
- AICS should react to the player's own improvement so as to not get steamrolled.
                
- AICS's success in combat should improve the island they are spawned from.
                
- An allegiance system where AICS spawned from a particular island are allied and will not fight each other.
            Islands
                
- Islands should have goals that guide the AICS belonging to them.
                
- Islands should be able to be negotiated with and be able to become friendly with the player.
                
- Islands should change and improve as the game progresses and in relation to the game events that have occurred.


## Areas for Development
Actors
    Islands
        Islands.rs
            
- implement population sizing
        Island_setup.rs
            
- Refactoring hacky loops
        on_turn_system.rs
            
- Refactor inefficient file reading
    population
        population.rs
            
- implement population function on island creation
            
- implement population changing
    ships
        mod.rs
            
- implement changing/mutable resources
                not sure why this isn't working
            
- implement ship interaction
console:
    
- fix functionality
entity_builder:
    
- move entity assembly of spawned ships here
    player.rs:
        
- file ready for implementation of proposed systems
generators:
    terrain_generators.rs
        
- remove testing console outputs
renderer:
    rendering_system.rs
        
- refactor to store player position so we don't need to access entitites set every render 
        
- implement instructions menu panel, right now there isn't anywhere that explains what's going on or how to interact beyond moving and quitting
    tcod_renderer.rs    
        
- fix invalid rendering. Currently edge case spawns player outside map, not sure what edge case is.
resources:
    
- refactor naming, confusing with in game resources sharing naming
systems:
    
- many systems unimplemented satisfactorily
    ai.rs:
    death_system.rs:
    execute_actions.rs:
    food_system.rs:
    interaction_system.rs:
    mod.rs:
ui:
    
- follows different pattern from rest of project
    init.rs:
    markers.rs:
    mod.rs:
    panel.rs:
    ui_systems.rs:
        
- refactor unclear switch statement, different cases defined by int, not commented with what is going on
        
- split ui_systems.rs into multiple files, not taking advantage of ECS as it could.
        
- honestly not bad for me of 5 years ago, I clearly have no idea what is bad practice, but its not horrible for such a complex idea. 
main.rs:
    
- some todos to address, generally pretty good
    
- add overview
file_io.rs:
    
- some dead code to remove
    
- generally don't mess with it
High Level Refactoring 
    
- map placement logic repeated in many places
    
- redo readme, running program guide outdated
    
- its unclear how the event channel is working, if at all
    
- In an effort to get a working demo, components and actors have been defined in generation component
