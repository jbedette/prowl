## Roadmap
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
    
event_channel:

generators:
    terrain_generators.rs
        - remove testing console outputs

input:

renderer:
    rendering_system.rs
        - refactor to store player position so we don't need to access entitites set every render 
        - implement instructions menu panel, right now there isn't anywhere that explains what's going on or how to interact beyond moving and quitting
    tcod_renderer.rs    
        - fix invalid rendering. Currently edge case spawns player outside map, not sure what edge case is.

resources:
    - refactor naming, confusing with in game resources sharing naming
    - update window generation, currently using deprecated code.

shared:
    application_root_dir.rs:
        - cut dead code, not sure what is relevant at the moment, needs cleaning
    normalize.rs:
        - dead code, but useful for development, cut later
    vector2.rs
        - 

systems:
    - many systems unimplemented
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
        - 
        - honestly not bad for me of 5 years ago, I clearly have no idea what is bad practice, but its not horrible for such a complex idea. 
main.rs:
    - some todos to address, generally pretty good
    - add overview
file_io.rs:
    - some dead code to remove
    - generally don't mess with it
dispatch:
    - not bad
    - add overview. ECS is weird, maybe a little walkthru


High Level Refactoring 
    - map placement logic repeated in many places
    - redo readme, running program guide outdated
    - its unclear how the event channel is working, if at all
    - In an effort to get a working demo, components and actors have been defined in generation component
    - It seems like the components folder might not be used. Investigate further. Are these components being redefined in each entity?

Systems to implement:
    - Resource management
    - Island association

// final writeup
## Intro:
    For this final Code Reading project, I selected a game demo that I worked on in the early stages of my degree. It is built in rust and uses the Specs Crate to manage an Entity Component System organization and the TCOD Crate for rendering the game. 

    Our project had a large refactoring near the end as we switched rendering libraries from Termion to TCOD. While our submission worked and compiled, the late switch lead to some nasty code structure issues. 

    In addition, I personally was far out of my depth in the project. I had taken the course very early on in my degree and my own knowledge was severly lacking in areas of structure and organization. My code, specifically the menu system which was my largest solo contribution has repetition, vague variables and some poor organizational decisions.

    Unfortunately in the time that has passed since this project was made, the TCOD library has been abandonded and results in build conflicts on every architecture I have tested except for Debian. This lead to some serious issues with testing some refactors I would like to make. On top of that, I haven't touched the Rust language in a meaningful way since working on this project. It has been a struggle, but Rust is just so great. Wonderful language.

    As a result, I spent the majority of my time writing comments about how each file works and how it is used. Refactoring proved difficult as I do not have easy access to an ubuntu machine on a 32 bit architecture. I did manage to identify some structural problems that could be sorted quickly and refactored the code to solve these issues.

## Decisions:
    Documentation:
        1. The first question I asked when I began work on this, was 'who will this work be for?'. Looking at the context of the codebase, which is a game that is/was a student project that ended with a proof of concept. The game, prowl, is not particularly close to a point where it would be used by a client, and thus I determined that my target audience would be another student or myself who would either be maintaining the aging code or doing some further development on the project. This means that:
            - The target audience for these comments is at a skill level and competence below a professional level. 
            - The target audience would be largely unfamiliar with an Entity Compoent system. ECS is an uncommon organization. While it shows up mainly in a GameDev context, it is also used in simulation and big data contexts. A student may have encountered ECS in any of these contexts, but the average student would likely have not encountered this before. So I wrote a small summary of ECS to guide a future Developer or Maintainer. 
            
        2. How to reach a future developer with documentation? As this is a game in development, a future developer needs to have an idea of where this is going. As such, I have prepared some readmes for the developers to peruse before begginning work.
            - A roadmap with the intended development direction
            - A readme for development
            - A readme for maitenence

        3. How make a complex codebase more accessible? As this is an ambitious game with many unfinished systems and unused components that are still intended to be used later, I chose to write a brief description of each file's usage. This excludes several game components that I felt were explicit enough in their naming that anyone familiar with a game would know the intended usage.  For example, the weapon component: while it is unimplemented, it seems apparent to me that a developer could infer what a weapon in a game would entail. 

    Commenting:
        1. Context is needed for all moving parts, writing brief overviews found in files and mod.rs files will help.

        2. Some fairly unclear and uncommented code blocks, write illustrative and descriptive comments for these.
    

## Summary of Code Improvements:
    1. Refactoring.
        - Moved initialization of Entities from mod.rs files to init.rs files. Best practices in Rust states that mod files should only contain exports of module members. 
        - Identified locations where further refactoring and reorganization is necessary. It would be wonderful to fix all of these problems, but some of them require a level of refactoring that would essentially demand a total rebuild of certain sections.  

    2. Bug Fixes.
        - Eliminated compilation warnings resulting from unused code and data safety.
        - Fixed changing input and output requirements resulting from crate updates and Rust language updates.

    3. Dead Code.
        - Removed many places where old testing code was commented out but left behind.
        
    4. Commenting.
        - Overviews. As the project is complex, I have written overview comments at the top of relevant files. This is intended to help a developer or maintainer interacting with the codebase for the first time. 
        - Code comments. There are quite a few places that code blocks were uncommented and unclear. I wrote illustrative comments to help a developer understand the what/where/why of the functions. There are many places where more clarity 
        - Left behind previously made 'todo' comments, they are useful guides for problem points.

    5. Documentation.
        - Wrote documentation for potential future developers and maintainers
            - created ROADMAP.md
            - created README_DEVELOPERS.md to provide some guidance about intended future development
            - created README_MAINTAINERS.md to highlight areas that need maitenence to facilitate the continued functioning of the program.
            - created DESIGN_OVERVIEW.md to give some reasoning about the program structure decisions. It is intended to be used as a guide for future decisions. 
            - updated README.md to allow easy access to new documentation and reflect the current deployment instructions.

## Summary of Problem Points:
    1. The effort to get a working demo resulted in convenient but hacky code repetition and poor organization of files.
        Status: Unresolved, outlined in README_MAINTAINERS.md
    2. We struggled implementing the termion crate. A late switch to the tcod crate led to us having some files and folders with unclear naming conventions and poor organization.
        Status: Unresolved, outlined in README_MAINTAINERS.md
    3. Running instructions are outdated.
        Status: Unresolved, outlined in README_MAINTAINERS.md
    4. Some library and language updates have broken the functions we used.
        Status: Resolved
    5. Code in later stages of development is largely uncommented. 
        Status: In progress, 
    6. Mod files contain initialization functions, refactor to move initialization and registration to new files.  
        Status: Unresolved
    7. Complilation Warnings
        Status: Resolved

## Problems Encountered during project:
    1. The TCOD crate used in this project was abandoned soon after we stopped development. It being outdated has resulted in our project being stranded.
    2. I haven't done anything in rust in a long time, and rust comes with a whole bucket of concerns about data safety that I have had to relearn in order to do the refactors needed.
    3. The rushed development of the project is evident. While we clearly had an organizational system, it was abandoned later in develpment in order to reach a state where we could present our work.
    4. I was having some git issues and so I created a new repository to work, this was a bad idea.

## Analysis of Project Proposal and Actual Work

In the proposal I identified these problems:

    1. The commenting is not explicit enough on two fronts.

        A. The work Matt did was organized fairly well and most functions were commented. However, comments usually were usually not explicit enough about function guts, variables are mysteriously named, and modules did not have readme's explaining flow.

        B. The work I did is a pile of uncommented spaghetti. Huge uncommented files everywhere.

    2. There's a fair amount of dead code to trim.

    3. It was a cool project and I'd like to revisit it.

My initial project plan was:

    1. Write readme's for each module. This will involve diving into the code and determining flow.

    2. Unravel my spaghetti in the menu system.

    3. Write more explicit comments for my menu system and improve menu variables to be more explicit.

    4. Write more explicit comments for Matt's procedural generation. Descriptors of where variables are coming from and Improve variable clarity in the procedural generation.

    5. Write more explicit comments for movement modules and improve variable clarity.

    6. Trim dead code.

As I worked in this, several things became apparent in relation to my initial evaluations:

    1.  There were more issues in organization than I had previously realized.

    2.  The work my project partner did was better commented than I had expected.

    3.  There was less dead code than was expected. Examining the codebase in the context of future developement meant that many sections of seemingly dead code were instead just unimplemented. As future development would be expanding on these sections and they generally are functioning as placeholders, it would not be correct to remove them.
 
Plan status:

    1. Write readme's for each module. This will involve diving into the code and determining flow.
        - Completed, opted to use overview comments in the mod files.

    2. Unravel my spaghetti in the menu system.
        - It turned out t 

    3. Write more explicit comments for my menu system and improve menu variables to be more explicit.

    4. Write more explicit comments for Matt's procedural generation. Descriptors of where variables are coming from and Improve variable clarity in the procedural generation.

    5. Write more explicit comments for movement modules and improve variable clarity.

    6. Trim dead code.


Changelog:
    files:
        actors:
            islands:
                island.rs:
                    did:
                        - removed commented code lines 9-10, population defined in population/population.rs
                        - added overview
                island_setup.rs
                        - added overview
                on_turn_system:
                        - added overview
            population:
                population.rs:
                        - added overview
            mod.rs:  
                    - added overview comment
                    - removed dead code line 9
                    - removed registration function
            init.rs:
                    - moved initialization and registration to this file
        components
            - mod.rs
                - added overview comment
                - removed initialziation functionality
            - init.rs
                - added file, moved initialization to here
        console
            - mod.rs
                - added overview comment
        shared
            random.rs:
                    - changed syntax for low and high values taken into range func
            vector2.rs
        generators
                - added overview
                - fixed broken code, simplex needed seed value passed
                - removed dead code 3,13,14-15,51-53,60, old testing commented code and dead design choices

        systems folder:
            ai.rs:
                - added overview comment
            death_system.rs:
                - added overview comment
            execute_actions.rs:
                - added overview comment
                problems:
                    dead code
                    inefficient code
            food_system.rs:
                problems:
                    - where is this separate from death_system.rs
            interaction_system.rs:
                problems:
                    - magic numbers unexplained
                    - not enough comments


        input folder:
            problems:
                failed implementations
            interaction_system.rs:
                - added overviwe comment
                - added comments to run describing function
        ui folder
            problems:
                uncommented
                unclear
            init.rs:
                did:
                    - added overview

            markers.rs:
            mod.rs:
            panel.rs:
            ui_systems.rs:
                did:
                    - removed dead testing code