# RELAY NOTES

## Test Realay with current implementation

```sql
-- with first
SELECT name FROM person ORDER BY name ASC LIMIT 4;
"name": "Andy"
"name": "Devy"
"name": "Dian"
"name": "Funi"
--with first and after - page 2
SELECT name FROM person WHERE name > "Funi" ORDER BY name ASC LIMIT 4;
"name": "Hulk"
"name": "Jack"
"name": "Jamie"
"name": "Jill"
--with first and after - page 2
SELECT name FROM person WHERE name > "Jill" ORDER BY name ASC LIMIT 4;
"name": "Jomo"
"name": "Joni"
"name": "Koakh"
"name": "Pelo"
--with first and after - page 3
SELECT name FROM person WHERE name > "Pelo" ORDER BY name ASC LIMIT 4;
"name": "Pete"
"name": "Peti"
"name": "Pini"
"name": "Tobie"

-- with first
SELECT count, count > 4 AS greater FROM (SELECT id, count() as count FROM (SELECT id FROM person ORDER BY name ASC LIMIT 4 + 1) GROUP BY ALL);
--with first and after - page 3
-- Pelu is the last item that have a next page, ex if we start in pelu we have pelu, pete,peti and pini, and next page was a page of one item with tobie only
SELECT count, count > 4 AS greater FROM (SELECT id, count() as count FROM (SELECT id FROM person WHERE name > "Pelo" ORDER BY name ASC LIMIT 4 + 1) GROUP BY ALL);


SELECT count, count > 4 AS greater FROM (SELECT id, count() as count FROM (SELECT id FROM person WHERE name < "Pini" ORDER BY name DESC LIMIT 4) GROUP BY ALL);
```

## With implicit ids

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

## With explicit ids

```json
"result": [
  {
    "age": 20,
    "id": "person:vu3cw39zhsxi79lk7zxi",
    "name": "Andy"
  },
  {
    "age": 19,
    "id": "person:lhejafwbn0s5gu46v1p6",
    "name": "Devy"
  },
  {
    "age": 20,
    "id": "person:uf2vo067tz3wypdg6319",
    "name": "Dian"
  },
  {
    "age": 24,
    "id": "person:26xpqg3kswf9ahaw6xhz",
    "name": "Funi"
  },
  {
    "age": 29,
    "id": "person:384b3u2lgteweywzd7kf",
    "name": "Hulk"
  },
  {
    "age": 28,
    "id": "person:cfmyv3jaxkbgspe6x2ql",
    "name": "Jack"
  },
  {
    "age": 40,
    "id": "person:mtr4uyxno5fv9pv94obn",
    "name": "Jamie"
  },
  {
    "age": 14,
    "id": "person:8mb52x1oxcajx45lszly",
    "name": "Jill"
  },
  {
    "age": null,
    "id": "person:rsr1sf20xt32ndmkzxu4",
    "name": "Jomo"
  },
  {
    "age": 56,
    "id": "person:wb7u1eaeyrqdubtuygbh",
    "name": "Joni"
  },
  {
    "age": 50,
    "id": "person:gf7vguo6i2gg88uwq425",
    "name": "Koakh"
  },
  {
    "age": 50,
    "id": "person:lakwe0ivk4jx244tuyjg",
    "name": "Pelo"
  },
  {
    "age": 16,
    "id": "person:b866unzsvcsye7lh7myn",
    "name": "Pete"
  },
  {
    "age": 20,
    "id": "person:glegkzqis5tv2vzdek5r",
    "name": "Peti"
  },
  {
    "age": 84,
    "id": "person:sg2z2pszx2l21o6mu667",
    "name": "Pini"
  },
  {
    "age": 30,
    "id": "person:uc09y1vunnyyofqglzz1",
    "name": "Tobie"
  }
],
```

```shell
$ SELECT name FROM person ORDER BY name LIMIT 4;
"name": "Andy"
"name": "Devy"
"name": "Dian"
"name": "Funi"

$ SELECT name FROM person WHERE name > 'Funi' ORDER BY name LIMIT 4;
"name": "Hulk"
"name": "Jack"
"name": "Jamie"
"name": "Jill"

$ SELECT name FROM person WHERE name > 'Jill' ORDER BY name LIMIT 4;
"name": "Jomo"
"name": "Joni"
"name": "Koakh"
"name": "Pelo"

$ SELECT name FROM person WHERE name > 'Pelo' ORDER BY name LIMIT 4;
"name": "Pete"
"name": "Peti"
"name": "Pini"
"name": "Tobie"
```
