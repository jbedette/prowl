## Intro:
    For this final Code Reading project, I selected a game demo that I worked on in the early stages of my degree. It is built in rust and uses the Specs Crate to manage an Entity Component System organization and the TCOD Crate for rendering the game. 

    Our project had a large refactoring near the end as we switched rendering libraries from Termion to TCOD. While our submission worked and compiled, the late switch lead to some nasty code structure issues. 

    In addition, I personally was far out of my depth in the project. I had taken the course very early on in my degree and my own knowledge was severly lacking in areas of structure and organization. My code, specifically the menu system which was my largest solo contribution has repetition, vague variables and some poor organizational decisions.

    Unfortunately in the time that has passed since this project was made, the TCOD library has been abandonded and results in build conflicts on every architecture I have tested except for Debian. This lead to some serious issues with testing some refactors I would like to make. On top of that, I haven't touched the Rust language in a meaningful way since working on this project. It has been a struggle, but Rust is just so great. Wonderful language.

    As a result, I spent the majority of my time writing comments about how each file works and how it is used. Refactoring proved difficult as I do not have easy access to an ubuntu machine on a 32 bit architecture. I did manage to identify some structural problems that could be sorted quickly and refactored the code to solve these issues.

    
## Analysis of Project Proposal, Plan, and Adaptation of the Plan to Reality of Codebase

### Initially Identified Problems

    1. The commenting is not explicit enough on two fronts.

        A. The work Matt did was organized fairly well and most functions were commented. However, comments usually were usually not explicit enough about function guts, variables are mysteriously named, and modules did not have readme's explaining flow.

        B. The work I did is a pile of uncommented spaghetti. Huge uncommented files everywhere.

    2. There's a fair amount of dead code to trim.

    3. It was a cool project and I'd like to revisit it.

### Initial Project Plan

    1. Write readme's for each module. This will involve diving into the code and determining flow.

    2. Unravel my spaghetti in the menu system.

    3. Write more explicit comments for my menu system and improve menu variables to be more explicit.

    4. Write more explicit comments for Matt's procedural generation. Descriptors of where variables are coming from and Improve variable clarity in the procedural generation.

    5. Write more explicit comments for movement modules and improve variable clarity.

    6. Trim dead code.

### Plan Divergence
As I worked in this, several things became apparent in relation to my initial evaluations:

    1.  There were more issues in organization than I had previously realized.

    2.  The work my project partner did was better commented than I had expected.

    3.  There was less dead code than was expected. Examining the codebase in the context of future developement meant that many sections of seemingly dead code were instead just unimplemented. As future development would be expanding on these sections and they generally are functioning as placeholders, it would not be correct to remove them.
 
### Plan V2

    Problems to solve:
        1. Determine target audience
        2. Code has insufficient comments for target audience
        3. Code has poor structure  
        4. Deployment instructions outdated
        5. No documentation of current problems
        6. No documentation of project direction
    
    Actions to take:
        1. Determine target audience, this will inform actions taken
        2. Pinpoint areas where comments are insufficient and take note of them.
            - Do modules have descriptions of what they are, what they do, and why they are necessary?
            - Does the code within the codebase have sufficiently illustrating comments describing variables and design decisions?
        3. Pinpoint areas where code structure are not adequate and take note of them.
            - Are naming conventions appropriate?
            - Have best practices been followed?
            - At a high level, are files located appropriately within file structure?
            - At a lower level, do files contain functionality that do not match the file's intended function?
        4. Create Documentation for audience.
            - What issues does a future developer need to know about?
            - What issues does a maintainer need to know about?
            - Where is the project intended to go?
        5. Compile project and locate any bugs or warnings.
        6. Fix any simple bugs and address warnings.
        7. Address list made in 2: 
            - Go to areas with insufficient comments and improve.
        8. Address list made in 3: 
            - Fix the simple refactoring issues.
            - Make note of larger issues in documentation for future improvement.
        9. Identify truly dead sections and cut them.
        10. Clean up refactoring notes.

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

    Refactoring:
        1. Establish what is dead code and what is exploratory code. We want to keep exploratory code.

        2. Determine what is reasonable to refactor versus what is a larger problem extending beyond the scope of this project. Rust is difficult to manage, some areas are too intertwined to unravel in a reasonable amount of time and will go against the spirit of the project which is reading the codebase and improving extensibility and documentation. Some of these refactoring problems will go into resuming full development of the project. 


## Summary of Code Improvements:
    1. Refactoring.
        - Moved initialization of Entities from mod.rs files to init.rs files. Best practices in Rust states that mod files should only contain exports of module members. 
        - Identified locations where further refactoring and reorganization is necessary. It would be wonderful to fix all of these problems, but some of them require a level of refactoring that would essentially demand a total rebuild of certain sections.  

    2. Bug Fixes.
        - Eliminated compilation warnings resulting from unused code and data safety.
        - Fixed changing input and output requirements resulting from crate updates and Rust language updates.

    3. Dead Code.
        - Removed many places where old testing code was commented out but left behind.
        - Removed dead files.
        
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

## Problems Encountered during project:
    1. The TCOD crate used in this project was abandoned soon after we stopped development. It being outdated has resulted in our project being stranded.
    2. I haven't done anything in rust in a long time, and rust comes with a whole bucket of concerns about data safety that I have had to relearn in order to do the refactors needed.
    3. The rushed development of the project is evident. While we clearly had an organizational system, it was abandoned later in develpment in order to reach a state where we could present our work.
    4. Our previous project plan was lost, and so I had to infer our desired development direction from the existing code.

## Conclusion

    While my initial evaluation of the problems in the codebase was naive, it was not entirely off base. In our initial work I was out of my depth and had found the project was byzantine structure and opaque in function. As a much stronger developer now, I find that the codebase was not nearly as daunting as I had once believed. That being said, a true refactoring and reorganization of the code base would have extended beyond the scope of this project. If I truly wanted to make this a workable codebase, I would start a fresh project and use the work done in this as a guide. There are simply too many areas where the code is poorly organized and the strictness of the Rust language makes quick refactoring and flexibility difficult. Revisiting this project, I am reminded of this article [Leaving Rust Gamedev After 3 Years](https://loglog.games/blog/leaving-rust-gamedev/). In this article the author identifies many of the same problem points I found with the Rust language. It simply isn't a friendly environment for rapid iteration, which in my experience is a key tenet of game development. 

    Having said this, I feel that I have done a satifactory job in analyzing this code base, providing a future developer or maintainer with adequate documentation and comments, fixing and improving the existing documentation, refactoring with the goal of promoting separation of concerns, and providing a set of places the codebase could be improved. I am leaving this project in a far better state than I found it. I may return to this project and build a more streamlined version.  




