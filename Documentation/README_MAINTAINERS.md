# For Maintainers
## Organization Concerns 

    File structure 
        - Generation 
            Currently some generation function is located in components
                - map.rs has map generation logic that should be moved into generators
                Todo: separate out map generation logic and relocate from the components module to the generation module.
        - Systems
            - Some differentiation between game interaction systems and game running systems is needed.
                There is a notional separation between systems that run the game and systems that make up the game.
                Right now, there is a confusing combination where both kinds of systems are spread across the codebase.
                Todo: separate the systems module into three submodules, input, rendering, and game
            - Currently some systems are within their modules
                - actors/islands/on_turn_system.rs
                - ui/ui_systems.rs
                - input/main_loop_systems.rs
                Todo: move these files into the appropriate systems subfolder
        - Naming confusion
            - Resources can refer to either the game resources (Wood,Food,Metal,Money) or the rendering resources (Window, Turns, Next Turn, etc.)
            Todo: While the relevant game resources are contained in components and referred to as game_resources, this is an unclear delineation from the resources module that contains functions relevant for the rendering and running of the actual game. I suggest that the resources module be renamed to runtime_control.

## Deployment Concerns

    The Readme instructions for running the game are outdated as a result of the TCOD crate being abandoned.
        - Windows
            Status: Non-functional in either case
        - OSX
            Apple now uses their own proprietary ARM architecture
            - ARM architecture
                Status: Non-functional
            - 32/64 bit architecture
                Status: Untested
        - Linux
            Status: Working
        Todo:
            This project needs serious work reestablishing deployment on all operating systems.
            Solutions:
                1. Trace the root of the problems in the TCOD library, it is possible that the current instructions could be fixed.
                2. Wrap the executable within a stripped down linux wrapper. This seems inefficient.
                3. Refactor the codebase to use an alternative rendering crate. Termion has had continuous development, likely a good candidate.

