```sql
SELECT id FROM person ORDER BY id;
"id": "person:andy"
"id": "person:devy"
"id": "person:dian"
"id": "person:funi"
"id": "person:hulk"
"id": "person:jack"
"id": "person:jamie"
"id": "person:jill"
"id": "person:jomo"
"id": "person:joni"
"id": "person:koakh"
"id": "person:pelu"
"id": "person:pete"
"id": "person:peti"
"id": "person:pini"
"id": "person:tobie"

SELECT id FROM person ORDER BY id LIMIT 4;
"id": "person:andy"
"id": "person:devy"
"id": "person:dian"
"id": "person:funi"

SELECT id FROM person WHERE id > person:funi ORDER BY id LIMIT 4;
"id": "person:hulk"
"id": "person:jack"
"id": "person:jamie"
"id": "person:jill"

SELECT id FROM person WHERE id > person:jill ORDER BY id LIMIT 4;
"id": "person:jomo"
"id": "person:joni"
"id": "person:koakh"
"id": "person:pelu"

SELECT id FROM person WHERE id > person:pelu ORDER BY id LIMIT 4;
"id": "person:pete"
"id": "person:peti"
"id": "person:pini"
"id": "person:tobie"

> SELECT name FROM person ORDER BY name;
"name": "Andy"
"name": "Devy"
"name": "Funi"
"name": "Hulk"
"name": "Jack"
"name": "Jack"
"name": "Jamie"
"name": "Jill"
"name": "Jomo"
"name": "Joni"
"name": "Koakh"
"name": "Pelo"
"name": "Pete"
"name": "Peti"
"name": "Pini"
"name": "Tobie"

SELECT name FROM person ORDER BY name LIMIT 4;
"name": "Andy"
"name": "Devy"
"name": "Funi"
"name": "Hulk"

SELECT name FROM person WHERE name > "Hulk" ORDER BY name LIMIT 4;
"name": "Jack"
"name": "Jack"
"name": "Jamie"
"name": "Jill"

SELECT name FROM person WHERE name > "Jill" ORDER BY name LIMIT 4;
"name": "Jomo"
"name": "Joni"
"name": "Koakh"
"name": "Pelo"

SELECT name FROM person WHERE name > "Pelo" ORDER BY name LIMIT 4;
"name": "Pete"
"name": "Peti"
"name": "Pini"
"name": "Tobie"

-- using desc and more that one order field

-- must include name for work
> SELECT id,name FROM person ORDER BY id DESC, name DESC;
[ 
  { 
    "result": [
      { 
        "id": "person:tobie",
        "name": "Tobie"
      },
      { 
        "id": "person:pini",
        "name": "Pini"
      },
      { 
        "id": "person:peti",
        "name": "Peti"
      },
      { 
        "id": "person:pete",
        "name": "Pete"
      },
      { 
        "id": "person:pelu",
        "name": "Pelo"
      },
      { 
        "id": "person:koakh",
        "name": "Koakh"
      },
      { 
        "id": "person:joni",
        "name": "Joni"
      },
      { 
        "id": "person:jomo",
        "name": "Jomo"
      },
      { 
        "id": "person:jill",
        "name": "Jill"
      },
      { 
        "id": "person:jamie",
        "name": "Jamie"
      },
      { 
        "id": "person:jack",
        "name": "Jack"
      },
      { 
        "id": "person:hulk",
        "name": "Hulk"
      },
      { 
        "id": "person:funi",
        "name": "Funi"
      },
      { 
        "id": "person:dian",
        "name": "Jack"
      },
      { 
        "id": "person:devy",
        "name": "Devy"
      },
      { 
        "id": "person:andy",
        "name": "Andy"
      }
    ],
    "status": "OK",
    "time": "154.016µs"
  }
]

> SELECT id,name FROM person ORDER BY id DESC, name DESC LIMIT 4;
[
  {
    "result": [
      {
        "id": "person:tobie",
        "name": "Tobie"
      },
      {
        "id": "person:pini",
        "name": "Pini"
      },
      {
        "id": "person:peti",
        "name": "Peti"
      },
      {
        "id": "person:pete",
        "name": "Pete"
      }
    ],
    "status": "OK",
    "time": "156.324µs"
  }
]

-- must use < here, when we use DESC
> SELECT id,name FROM person WHERE id < person:pete AND name < "Pete" ORDER BY id DESC, name DESC LIMIT 4;
[
  {
    "result": [
      {
        "id": "person:pelu",
        "name": "Pelo"
      },
      {
        "id": "person:koakh",
        "name": "Koakh"
      },
      {
        "id": "person:joni",
        "name": "Joni"
      },
      {
        "id": "person:jomo",
        "name": "Jomo"
      }
    ],
    "status": "OK",
    "time": "142.802µs"
  }
]

> SELECT id,name FROM person WHERE id < person:jomo AND name < "Jomo" ORDER BY id DESC, name DESC LIMIT 4;
[
  {
    "result": [
      {
        "id": "person:jill",
        "name": "Jill"
      },
      {
        "id": "person:jamie",
        "name": "Jamie"
      },
      {
        "id": "person:jack",
        "name": "Jack"
      },
      {
        "id": "person:hulk",
        "name": "Hulk"
      }
    ],
    "status": "OK",
    "time": "228.22µs"
  }
]

> SELECT id,name FROM person WHERE id < person:hulk AND name < "Hulk" ORDER BY id DESC, name DESC LIMIT 4;
[
  {
    "result": [
      {
        "id": "person:funi",
        "name": "Funi"
      },
      {
        "id": "person:dian",
        "name": "Dian"
      },
      {
        "id": "person:devy",
        "name": "Devy"
      },
      {
        "id": "person:andy",
        "name": "Andy"
      }
    ],
    "status": "OK",
    "time": "145.169µs"
  }
]
```
