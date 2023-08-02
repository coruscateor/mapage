
{
  test {
    value
  }
}

https://graphql.org/learn/queries/

http://localhost:8000/

mutation CreateStoredObjectMut($id: IdentifierInput!, $ty: NonGenericType!) {
  createStoredObjectMut(identifier: $id, theType: $ty) {
     value
  }
}

{
  "id": {
    "name": "test"
    },
  "ty": "BOOL"
}

//

query GetValue($id: IdentifierInput!) {

  getValue(identifier: $id) {

    __typename

    ... on BoolValue
    {
      value
    }
    ... on CharValue
    {
      value
    }
    ... on F32Value
    {
      value
    }
    ... on F64Value
    {
      value
    }
    ... on I8Value
    {
      value
    }
    ... on I16Value
    {
      value
    }
    ... on I32Value
    {
      value
    }
    ... on I64Value
    {
      value
    }
    ... on I128Value
    {
      value
    }
    ... on IsizeValue
    {
      value
    }
    ... on U8Value
    {
      value
    }
    ... on U16Value
    {
      value
    }
    ... on U32Value
    {
      value
    }
    ... on U64Value
    {
      value
    }
    ... on U128Value
    {
      value
    }
    ... on UsizeValue
    {
      value
    }
    ... on StringValue
    {
      value
    }
    ... on UnitValue
    {
      value
    }
    ... on Identifier
    {
      name,
      namespace
    }
 
  }

}

{
  "id": {
    "name": "test"
    }
}

Result:

{
  "data": {
    "getValue": {
      "__typename": "BoolValue",
      "value": false
    }
  }
}

//

query GetBool($id: IdentifierInput!) {
  getBool(identifier: $id)
}

{
  "id": {
    "name": "test"
    }
}

//

mutation SetBool($id: IdentifierInput!, $in: Boolean) {
  setBool(identifier: $id, input: $in) {
     value
  }
}

{
  "id": {
    "name": "test"
  },
  "in": true
}

Result:

{
  "data": null,
  "errors": [
    {
      "message": "Invalid Opertation: Instance is immuatable",
      "locations": [
        {
          "line": 2,
          "column": 3
        }
      ],
      "path": [
        "setBool"
      ]
    }
  ]
}

