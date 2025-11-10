# redactr-rs
> (WIP)

## Structure
```mermaid
graph TD
    subgraph CoreLib
        Lib[lib.rs]  
        Types[types.rs]  
        Errors[errors.rs]  
        EnginesSub[engines/]
    end

    subgraph Engines
        RegexEngine[regex.rs<br/>RegexRedactor<'p>]  
        ACEngine[aho.rs<br/>ACRedactor<'p>]
    end

    CLI[main.rs]  

    Lib --> Types  
    Lib --> Errors  
    Lib --> EnginesSub  
    EnginesSub --> RegexEngine  
    EnginesSub --> ACEngine  
    CLI --> Lib  

    Types --> Span  
    Types --> Finding  
    Types --> Report  
    Errors --> RedactError  
    EnginesSub --> RedactorTrait[«trait» Redactor<'p>]
    RegexEngine -->|impl| RedactorTrait  
    ACEngine -->|impl| RedactorTrait  
```
