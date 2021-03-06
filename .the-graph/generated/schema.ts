// THIS IS AN AUTOGENERATED FILE. DO NOT EDIT THIS FILE DIRECTLY.

import {
  TypedMap,
  Entity,
  Value,
  ValueKind,
  store,
  Bytes,
  BigInt,
  BigDecimal
} from "@graphprotocol/graph-ts";

export class TokenMetadata extends Entity {
  constructor(id: string) {
    super();
    this.set("id", Value.fromString(id));

    this.set("tokenId", Value.fromString(""));
  }

  save(): void {
    let id = this.get("id");
    assert(id != null, "Cannot save TokenMetadata entity without an ID");
    if (id) {
      assert(
        id.kind == ValueKind.STRING,
        "Cannot save TokenMetadata entity with non-string ID. " +
          'Considering using .toHex() to convert the "id" to a string.'
      );
      store.set("TokenMetadata", id.toString(), this);
    }
  }

  static load(id: string): TokenMetadata | null {
    return changetype<TokenMetadata | null>(store.get("TokenMetadata", id));
  }

  get id(): string {
    let value = this.get("id");
    return value!.toString();
  }

  set id(value: string) {
    this.set("id", Value.fromString(value));
  }

  get title(): string | null {
    let value = this.get("title");
    if (!value || value.kind == ValueKind.NULL) {
      return null;
    } else {
      return value.toString();
    }
  }

  set title(value: string | null) {
    if (!value) {
      this.unset("title");
    } else {
      this.set("title", Value.fromString(<string>value));
    }
  }

  get decsription(): string | null {
    let value = this.get("decsription");
    if (!value || value.kind == ValueKind.NULL) {
      return null;
    } else {
      return value.toString();
    }
  }

  set decsription(value: string | null) {
    if (!value) {
      this.unset("decsription");
    } else {
      this.set("decsription", Value.fromString(<string>value));
    }
  }

  get media(): string | null {
    let value = this.get("media");
    if (!value || value.kind == ValueKind.NULL) {
      return null;
    } else {
      return value.toString();
    }
  }

  set media(value: string | null) {
    if (!value) {
      this.unset("media");
    } else {
      this.set("media", Value.fromString(<string>value));
    }
  }

  get tokenId(): string {
    let value = this.get("tokenId");
    return value!.toString();
  }

  set tokenId(value: string) {
    this.set("tokenId", Value.fromString(value));
  }

  get tokens(): Array<string> {
    let value = this.get("tokens");
    return value!.toStringArray();
  }

  set tokens(value: Array<string>) {
    this.set("tokens", Value.fromStringArray(value));
  }
}

export class Token extends Entity {
  constructor(id: string) {
    super();
    this.set("id", Value.fromString(id));

    this.set("tokenId", Value.fromString(""));
    this.set("ownerId", Value.fromString(""));
    this.set("owner", Value.fromString(""));
  }

  save(): void {
    let id = this.get("id");
    assert(id != null, "Cannot save Token entity without an ID");
    if (id) {
      assert(
        id.kind == ValueKind.STRING,
        "Cannot save Token entity with non-string ID. " +
          'Considering using .toHex() to convert the "id" to a string.'
      );
      store.set("Token", id.toString(), this);
    }
  }

  static load(id: string): Token | null {
    return changetype<Token | null>(store.get("Token", id));
  }

  get id(): string {
    let value = this.get("id");
    return value!.toString();
  }

  set id(value: string) {
    this.set("id", Value.fromString(value));
  }

  get tokenId(): string {
    let value = this.get("tokenId");
    return value!.toString();
  }

  set tokenId(value: string) {
    this.set("tokenId", Value.fromString(value));
  }

  get collection(): string | null {
    let value = this.get("collection");
    if (!value || value.kind == ValueKind.NULL) {
      return null;
    } else {
      return value.toString();
    }
  }

  set collection(value: string | null) {
    if (!value) {
      this.unset("collection");
    } else {
      this.set("collection", Value.fromString(<string>value));
    }
  }

  get tokenType(): string | null {
    let value = this.get("tokenType");
    if (!value || value.kind == ValueKind.NULL) {
      return null;
    } else {
      return value.toString();
    }
  }

  set tokenType(value: string | null) {
    if (!value) {
      this.unset("tokenType");
    } else {
      this.set("tokenType", Value.fromString(<string>value));
    }
  }

  get tokenSubType(): string | null {
    let value = this.get("tokenSubType");
    if (!value || value.kind == ValueKind.NULL) {
      return null;
    } else {
      return value.toString();
    }
  }

  set tokenSubType(value: string | null) {
    if (!value) {
      this.unset("tokenSubType");
    } else {
      this.set("tokenSubType", Value.fromString(<string>value));
    }
  }

  get rarity(): string | null {
    let value = this.get("rarity");
    if (!value || value.kind == ValueKind.NULL) {
      return null;
    } else {
      return value.toString();
    }
  }

  set rarity(value: string | null) {
    if (!value) {
      this.unset("rarity");
    } else {
      this.set("rarity", Value.fromString(<string>value));
    }
  }

  get bindToOwner(): boolean {
    let value = this.get("bindToOwner");
    return value!.toBoolean();
  }

  set bindToOwner(value: boolean) {
    this.set("bindToOwner", Value.fromBoolean(value));
  }

  get locked(): boolean {
    let value = this.get("locked");
    return value!.toBoolean();
  }

  set locked(value: boolean) {
    this.set("locked", Value.fromBoolean(value));
  }

  get ownerId(): string {
    let value = this.get("ownerId");
    return value!.toString();
  }

  set ownerId(value: string) {
    this.set("ownerId", Value.fromString(value));
  }

  get tokenMetadataId(): string | null {
    let value = this.get("tokenMetadataId");
    if (!value || value.kind == ValueKind.NULL) {
      return null;
    } else {
      return value.toString();
    }
  }

  set tokenMetadataId(value: string | null) {
    if (!value) {
      this.unset("tokenMetadataId");
    } else {
      this.set("tokenMetadataId", Value.fromString(<string>value));
    }
  }

  get mintId(): string | null {
    let value = this.get("mintId");
    if (!value || value.kind == ValueKind.NULL) {
      return null;
    } else {
      return value.toString();
    }
  }

  set mintId(value: string | null) {
    if (!value) {
      this.unset("mintId");
    } else {
      this.set("mintId", Value.fromString(<string>value));
    }
  }

  get fractionationId(): string | null {
    let value = this.get("fractionationId");
    if (!value || value.kind == ValueKind.NULL) {
      return null;
    } else {
      return value.toString();
    }
  }

  set fractionationId(value: string | null) {
    if (!value) {
      this.unset("fractionationId");
    } else {
      this.set("fractionationId", Value.fromString(<string>value));
    }
  }

  get owner(): string {
    let value = this.get("owner");
    return value!.toString();
  }

  set owner(value: string) {
    this.set("owner", Value.fromString(value));
  }

  get tokenMetadata(): string | null {
    let value = this.get("tokenMetadata");
    if (!value || value.kind == ValueKind.NULL) {
      return null;
    } else {
      return value.toString();
    }
  }

  set tokenMetadata(value: string | null) {
    if (!value) {
      this.unset("tokenMetadata");
    } else {
      this.set("tokenMetadata", Value.fromString(<string>value));
    }
  }

  get mint(): string | null {
    let value = this.get("mint");
    if (!value || value.kind == ValueKind.NULL) {
      return null;
    } else {
      return value.toString();
    }
  }

  set mint(value: string | null) {
    if (!value) {
      this.unset("mint");
    } else {
      this.set("mint", Value.fromString(<string>value));
    }
  }

  get fractionation(): string | null {
    let value = this.get("fractionation");
    if (!value || value.kind == ValueKind.NULL) {
      return null;
    } else {
      return value.toString();
    }
  }

  set fractionation(value: string | null) {
    if (!value) {
      this.unset("fractionation");
    } else {
      this.set("fractionation", Value.fromString(<string>value));
    }
  }
}

export class Account extends Entity {
  constructor(id: string) {
    super();
    this.set("id", Value.fromString(id));

    this.set("balance", Value.fromBigInt(BigInt.zero()));
    this.set("totalPaySalesPrice", Value.fromBigDecimal(BigDecimal.zero()));
    this.set("totalPayRentsPrice", Value.fromBigDecimal(BigDecimal.zero()));
    this.set("totalMintsPrice", Value.fromBigDecimal(BigDecimal.zero()));
  }

  save(): void {
    let id = this.get("id");
    assert(id != null, "Cannot save Account entity without an ID");
    if (id) {
      assert(
        id.kind == ValueKind.STRING,
        "Cannot save Account entity with non-string ID. " +
          'Considering using .toHex() to convert the "id" to a string.'
      );
      store.set("Account", id.toString(), this);
    }
  }

  static load(id: string): Account | null {
    return changetype<Account | null>(store.get("Account", id));
  }

  get id(): string {
    let value = this.get("id");
    return value!.toString();
  }

  set id(value: string) {
    this.set("id", Value.fromString(value));
  }

  get balance(): BigInt {
    let value = this.get("balance");
    return value!.toBigInt();
  }

  set balance(value: BigInt) {
    this.set("balance", Value.fromBigInt(value));
  }

  get totalTransfers(): i32 {
    let value = this.get("totalTransfers");
    return value!.toI32();
  }

  set totalTransfers(value: i32) {
    this.set("totalTransfers", Value.fromI32(value));
  }

  get currentSales(): i32 {
    let value = this.get("currentSales");
    return value!.toI32();
  }

  set currentSales(value: i32) {
    this.set("currentSales", Value.fromI32(value));
  }

  get currentRents(): i32 {
    let value = this.get("currentRents");
    return value!.toI32();
  }

  set currentRents(value: i32) {
    this.set("currentRents", Value.fromI32(value));
  }

  get currentNfts(): i32 {
    let value = this.get("currentNfts");
    return value!.toI32();
  }

  set currentNfts(value: i32) {
    this.set("currentNfts", Value.fromI32(value));
  }

  get totalSales(): i32 {
    let value = this.get("totalSales");
    return value!.toI32();
  }

  set totalSales(value: i32) {
    this.set("totalSales", Value.fromI32(value));
  }

  get totalPaySales(): i32 {
    let value = this.get("totalPaySales");
    return value!.toI32();
  }

  set totalPaySales(value: i32) {
    this.set("totalPaySales", Value.fromI32(value));
  }

  get totalPaySalesPrice(): BigDecimal {
    let value = this.get("totalPaySalesPrice");
    return value!.toBigDecimal();
  }

  set totalPaySalesPrice(value: BigDecimal) {
    this.set("totalPaySalesPrice", Value.fromBigDecimal(value));
  }

  get totalRents(): i32 {
    let value = this.get("totalRents");
    return value!.toI32();
  }

  set totalRents(value: i32) {
    this.set("totalRents", Value.fromI32(value));
  }

  get totalPayRents(): i32 {
    let value = this.get("totalPayRents");
    return value!.toI32();
  }

  set totalPayRents(value: i32) {
    this.set("totalPayRents", Value.fromI32(value));
  }

  get totalPayRentsPrice(): BigDecimal {
    let value = this.get("totalPayRentsPrice");
    return value!.toBigDecimal();
  }

  set totalPayRentsPrice(value: BigDecimal) {
    this.set("totalPayRentsPrice", Value.fromBigDecimal(value));
  }

  get totalMints(): i32 {
    let value = this.get("totalMints");
    return value!.toI32();
  }

  set totalMints(value: i32) {
    this.set("totalMints", Value.fromI32(value));
  }

  get totalMintsPrice(): BigDecimal {
    let value = this.get("totalMintsPrice");
    return value!.toBigDecimal();
  }

  set totalMintsPrice(value: BigDecimal) {
    this.set("totalMintsPrice", Value.fromBigDecimal(value));
  }

  get totalBurn(): i32 {
    let value = this.get("totalBurn");
    return value!.toI32();
  }

  set totalBurn(value: i32) {
    this.set("totalBurn", Value.fromI32(value));
  }

  get tokens(): Array<string> {
    let value = this.get("tokens");
    return value!.toStringArray();
  }

  set tokens(value: Array<string>) {
    this.set("tokens", Value.fromStringArray(value));
  }

  get rents(): Array<string> {
    let value = this.get("rents");
    return value!.toStringArray();
  }

  set rents(value: Array<string>) {
    this.set("rents", Value.fromStringArray(value));
  }

  get sales(): Array<string> {
    let value = this.get("sales");
    return value!.toStringArray();
  }

  set sales(value: Array<string>) {
    this.set("sales", Value.fromStringArray(value));
  }

  get fractionations(): Array<string> {
    let value = this.get("fractionations");
    return value!.toStringArray();
  }

  set fractionations(value: Array<string>) {
    this.set("fractionations", Value.fromStringArray(value));
  }
}

export class Mint extends Entity {
  constructor(id: string) {
    super();
    this.set("id", Value.fromString(id));

    this.set("name", Value.fromString(""));
    this.set("price", Value.fromString(""));
  }

  save(): void {
    let id = this.get("id");
    assert(id != null, "Cannot save Mint entity without an ID");
    if (id) {
      assert(
        id.kind == ValueKind.STRING,
        "Cannot save Mint entity with non-string ID. " +
          'Considering using .toHex() to convert the "id" to a string.'
      );
      store.set("Mint", id.toString(), this);
    }
  }

  static load(id: string): Mint | null {
    return changetype<Mint | null>(store.get("Mint", id));
  }

  get id(): string {
    let value = this.get("id");
    return value!.toString();
  }

  set id(value: string) {
    this.set("id", Value.fromString(value));
  }

  get name(): string {
    let value = this.get("name");
    return value!.toString();
  }

  set name(value: string) {
    this.set("name", Value.fromString(value));
  }

  get amount(): i32 {
    let value = this.get("amount");
    return value!.toI32();
  }

  set amount(value: i32) {
    this.set("amount", Value.fromI32(value));
  }

  get price(): string {
    let value = this.get("price");
    return value!.toString();
  }

  set price(value: string) {
    this.set("price", Value.fromString(value));
  }

  get buyMax(): i32 {
    let value = this.get("buyMax");
    return value!.toI32();
  }

  set buyMax(value: i32) {
    this.set("buyMax", Value.fromI32(value));
  }

  get perTransactionMin(): i32 {
    let value = this.get("perTransactionMin");
    return value!.toI32();
  }

  set perTransactionMin(value: i32) {
    this.set("perTransactionMin", Value.fromI32(value));
  }

  get perTransactionMax(): i32 {
    let value = this.get("perTransactionMax");
    return value!.toI32();
  }

  set perTransactionMax(value: i32) {
    this.set("perTransactionMax", Value.fromI32(value));
  }

  get notMinted(): i32 {
    let value = this.get("notMinted");
    return value!.toI32();
  }

  set notMinted(value: i32) {
    this.set("notMinted", Value.fromI32(value));
  }

  get locked(): boolean {
    let value = this.get("locked");
    return value!.toBoolean();
  }

  set locked(value: boolean) {
    this.set("locked", Value.fromBoolean(value));
  }

  get startDate(): BigInt | null {
    let value = this.get("startDate");
    if (!value || value.kind == ValueKind.NULL) {
      return null;
    } else {
      return value.toBigInt();
    }
  }

  set startDate(value: BigInt | null) {
    if (!value) {
      this.unset("startDate");
    } else {
      this.set("startDate", Value.fromBigInt(<BigInt>value));
    }
  }

  get tokens(): Array<string> {
    let value = this.get("tokens");
    return value!.toStringArray();
  }

  set tokens(value: Array<string>) {
    this.set("tokens", Value.fromStringArray(value));
  }
}

export class Fractionation extends Entity {
  constructor(id: string) {
    super();
    this.set("id", Value.fromString(id));

    this.set("tokenId", Value.fromString(""));
    this.set("token", Value.fromString(""));
    this.set("parts", Value.fromStringArray(new Array(0)));
    this.set("ownerId", Value.fromString(""));
    this.set("owner", Value.fromString(""));
  }

  save(): void {
    let id = this.get("id");
    assert(id != null, "Cannot save Fractionation entity without an ID");
    if (id) {
      assert(
        id.kind == ValueKind.STRING,
        "Cannot save Fractionation entity with non-string ID. " +
          'Considering using .toHex() to convert the "id" to a string.'
      );
      store.set("Fractionation", id.toString(), this);
    }
  }

  static load(id: string): Fractionation | null {
    return changetype<Fractionation | null>(store.get("Fractionation", id));
  }

  get id(): string {
    let value = this.get("id");
    return value!.toString();
  }

  set id(value: string) {
    this.set("id", Value.fromString(value));
  }

  get tokenId(): string {
    let value = this.get("tokenId");
    return value!.toString();
  }

  set tokenId(value: string) {
    this.set("tokenId", Value.fromString(value));
  }

  get token(): string {
    let value = this.get("token");
    return value!.toString();
  }

  set token(value: string) {
    this.set("token", Value.fromString(value));
  }

  get parts(): Array<string> {
    let value = this.get("parts");
    return value!.toStringArray();
  }

  set parts(value: Array<string>) {
    this.set("parts", Value.fromStringArray(value));
  }

  get ownerId(): string {
    let value = this.get("ownerId");
    return value!.toString();
  }

  set ownerId(value: string) {
    this.set("ownerId", Value.fromString(value));
  }

  get owner(): string {
    let value = this.get("owner");
    return value!.toString();
  }

  set owner(value: string) {
    this.set("owner", Value.fromString(value));
  }

  get tokens(): Array<string> {
    let value = this.get("tokens");
    return value!.toStringArray();
  }

  set tokens(value: Array<string>) {
    this.set("tokens", Value.fromStringArray(value));
  }
}

export class Rent extends Entity {
  constructor(id: string) {
    super();
    this.set("id", Value.fromString(id));

    this.set("tokenId", Value.fromString(""));
    this.set("ownerId", Value.fromString(""));
    this.set("pricePerHour", Value.fromString(""));
    this.set("owner", Value.fromString(""));
    this.set("token", Value.fromString(""));
  }

  save(): void {
    let id = this.get("id");
    assert(id != null, "Cannot save Rent entity without an ID");
    if (id) {
      assert(
        id.kind == ValueKind.STRING,
        "Cannot save Rent entity with non-string ID. " +
          'Considering using .toHex() to convert the "id" to a string.'
      );
      store.set("Rent", id.toString(), this);
    }
  }

  static load(id: string): Rent | null {
    return changetype<Rent | null>(store.get("Rent", id));
  }

  get id(): string {
    let value = this.get("id");
    return value!.toString();
  }

  set id(value: string) {
    this.set("id", Value.fromString(value));
  }

  get tokenId(): string {
    let value = this.get("tokenId");
    return value!.toString();
  }

  set tokenId(value: string) {
    this.set("tokenId", Value.fromString(value));
  }

  get ownerId(): string {
    let value = this.get("ownerId");
    return value!.toString();
  }

  set ownerId(value: string) {
    this.set("ownerId", Value.fromString(value));
  }

  get pricePerHour(): string {
    let value = this.get("pricePerHour");
    return value!.toString();
  }

  set pricePerHour(value: string) {
    this.set("pricePerHour", Value.fromString(value));
  }

  get minTime(): i32 {
    let value = this.get("minTime");
    return value!.toI32();
  }

  set minTime(value: i32) {
    this.set("minTime", Value.fromI32(value));
  }

  get maxTime(): i32 {
    let value = this.get("maxTime");
    return value!.toI32();
  }

  set maxTime(value: i32) {
    this.set("maxTime", Value.fromI32(value));
  }

  get endedAt(): i32 {
    let value = this.get("endedAt");
    return value!.toI32();
  }

  set endedAt(value: i32) {
    this.set("endedAt", Value.fromI32(value));
  }

  get createdAt(): i32 {
    let value = this.get("createdAt");
    return value!.toI32();
  }

  set createdAt(value: i32) {
    this.set("createdAt", Value.fromI32(value));
  }

  get owner(): string {
    let value = this.get("owner");
    return value!.toString();
  }

  set owner(value: string) {
    this.set("owner", Value.fromString(value));
  }

  get token(): string {
    let value = this.get("token");
    return value!.toString();
  }

  set token(value: string) {
    this.set("token", Value.fromString(value));
  }
}

export class Sale extends Entity {
  constructor(id: string) {
    super();
    this.set("id", Value.fromString(id));

    this.set("ownerId", Value.fromString(""));
    this.set("nftContractId", Value.fromString(""));
    this.set("tokenId", Value.fromString(""));
    this.set("owner", Value.fromString(""));
    this.set("token", Value.fromString(""));
  }

  save(): void {
    let id = this.get("id");
    assert(id != null, "Cannot save Sale entity without an ID");
    if (id) {
      assert(
        id.kind == ValueKind.STRING,
        "Cannot save Sale entity with non-string ID. " +
          'Considering using .toHex() to convert the "id" to a string.'
      );
      store.set("Sale", id.toString(), this);
    }
  }

  static load(id: string): Sale | null {
    return changetype<Sale | null>(store.get("Sale", id));
  }

  get id(): string {
    let value = this.get("id");
    return value!.toString();
  }

  set id(value: string) {
    this.set("id", Value.fromString(value));
  }

  get ownerId(): string {
    let value = this.get("ownerId");
    return value!.toString();
  }

  set ownerId(value: string) {
    this.set("ownerId", Value.fromString(value));
  }

  get nftContractId(): string {
    let value = this.get("nftContractId");
    return value!.toString();
  }

  set nftContractId(value: string) {
    this.set("nftContractId", Value.fromString(value));
  }

  get tokenId(): string {
    let value = this.get("tokenId");
    return value!.toString();
  }

  set tokenId(value: string) {
    this.set("tokenId", Value.fromString(value));
  }

  get isAuction(): boolean {
    let value = this.get("isAuction");
    return value!.toBoolean();
  }

  set isAuction(value: boolean) {
    this.set("isAuction", Value.fromBoolean(value));
  }

  get createdAt(): i32 {
    let value = this.get("createdAt");
    return value!.toI32();
  }

  set createdAt(value: i32) {
    this.set("createdAt", Value.fromI32(value));
  }

  get owner(): string {
    let value = this.get("owner");
    return value!.toString();
  }

  set owner(value: string) {
    this.set("owner", Value.fromString(value));
  }

  get token(): string {
    let value = this.get("token");
    return value!.toString();
  }

  set token(value: string) {
    this.set("token", Value.fromString(value));
  }

  get conditions(): Array<string> {
    let value = this.get("conditions");
    return value!.toStringArray();
  }

  set conditions(value: Array<string>) {
    this.set("conditions", Value.fromStringArray(value));
  }
}

export class SaleCondition extends Entity {
  constructor(id: string) {
    super();
    this.set("id", Value.fromString(id));

    this.set("ftTokenId", Value.fromString(""));
    this.set("price", Value.fromString(""));
    this.set("sale", Value.fromString(""));
    this.set("saleId", Value.fromString(""));
  }

  save(): void {
    let id = this.get("id");
    assert(id != null, "Cannot save SaleCondition entity without an ID");
    if (id) {
      assert(
        id.kind == ValueKind.STRING,
        "Cannot save SaleCondition entity with non-string ID. " +
          'Considering using .toHex() to convert the "id" to a string.'
      );
      store.set("SaleCondition", id.toString(), this);
    }
  }

  static load(id: string): SaleCondition | null {
    return changetype<SaleCondition | null>(store.get("SaleCondition", id));
  }

  get id(): string {
    let value = this.get("id");
    return value!.toString();
  }

  set id(value: string) {
    this.set("id", Value.fromString(value));
  }

  get ftTokenId(): string {
    let value = this.get("ftTokenId");
    return value!.toString();
  }

  set ftTokenId(value: string) {
    this.set("ftTokenId", Value.fromString(value));
  }

  get price(): string {
    let value = this.get("price");
    return value!.toString();
  }

  set price(value: string) {
    this.set("price", Value.fromString(value));
  }

  get sale(): string {
    let value = this.get("sale");
    return value!.toString();
  }

  set sale(value: string) {
    this.set("sale", Value.fromString(value));
  }

  get saleId(): string {
    let value = this.get("saleId");
    return value!.toString();
  }

  set saleId(value: string) {
    this.set("saleId", Value.fromString(value));
  }
}
