# server


## Database ER Diagram
```mermaid
erDiagram
    Devices {
        int id PK
        string name
    }
    Configuration {
        int id PK
        int deviceid FK
    }
    Data {
        int id PK
        int deviceid FK
        float value
        time timestamp
    }
```
