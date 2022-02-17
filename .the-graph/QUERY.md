Tokens
```graphql
fragment TokenMetadata on Metadata {
    id
    title
    decsription
    media
}
fragment Token on Token {
    id
    rarity
    tokenType
    bindToOwner
    collection
    locked
    tokenMetadata {
        ...TokenMetadata
    }
}

{
  tokens(first:5, skip:5) {
    ...Token
  }
}
```

Fractionations
```graphql
fragment TokenMetadata on Metadata {
    id
    title
    decsription
    media
}
fragment Token on Token {
    id
    rarity
    tokenType
    bindToOwner
    collection
    locked
    tokenMetadata {
        ...TokenMetadata
    }
}
fragment Fractionation on Fractionation {
    id
    tokenId
    ownerId
    parts
    token {
      ...Token
    }
}

{
  fractionations(first:5, skip:5) {
    ...Fractionation
  }
}
```

Sales
```graphql
fragment SaleCondition on SaleCondition {
    id
    ftTokenId
    price
}
fragment TokenMetadata on Metadata {
    id
    title
    decsription
    media
}
fragment Token on Token {
    id
    rarity
    tokenType
    bindToOwner
    collection
    locked
    tokenMetadata {
        ...TokenMetadata
    }
}
fragment Sale on Sale {
    id
    ownerId
    isAuction
    createdAt
    conditions {
      ...SaleCondition
    }
    token {
     ...Token
    }
}

{
  sales(first: 5, skip: 5) {
    ...Sale
  }
}
```

Rents
```graphql
fragment TokenMetadata on Metadata {
    id
    title
    decsription
    media
}
fragment Token on Token {
    id
    rarity
    tokenType
    bindToOwner
    collection
    locked
    tokenMetadata {
        ...TokenMetadata
    }
}
fragment Rent on Rent {
    id
    tokenId
    ownerId
    pricePerHour
    minTime
    maxTime
    endedAt
    createdAt
    token {
      ...Token
    }
}

{
  rents(first: 5, skip:5) {
   ...Rent
  }
}
```

Mints
```graphql
fragment Mint on Mint {
    id
    locked
    notMinted
    perTransactionMin
    perTransactionMax
    price
    name
    startDate
    amount
}

{
  mints(first:10) {
    ...Mint
  }
}
```

Accounts
```graphql
fragment Account on Account {
    id
    balance
    totalBurn
    totalSales
    totalRents
    totalMints
    totalPaySales
    totalPayRents
    totalTransfers
    totalMintsPrice
    totalPaySalesPrice
    totalPayRentsPrice
    currentNfts
    currentSales
    currentRents
}

{
  accounts(first: 5) {
    ...Account
    tokens {
      tokenId
    }
    fractionations {
      tokenId
    }
    sales {
      tokenId
    }
    rents {
      tokenId
    }
  }
}
```

