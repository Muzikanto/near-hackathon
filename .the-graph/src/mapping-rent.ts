import { near, store } from "@graphprotocol/graph-ts"
import { log } from '@graphprotocol/graph-ts'
import { Account, Rent } from '../generated/schema';
import { parseEvent } from "./utils";
import { getOrCreateAccount } from "./helpers/account";
import { BigDecimal } from "@graphprotocol/graph-ts/index";

export function handleRent(
  receipt: near.ReceiptWithOutcome
): void {
  const actions = receipt.receipt.actions;
  for (let i = 0; i < actions.length; i++) {
    handleAction(actions[i], receipt)
  }
}

function handleAction(
  action: near.ActionValue,
  receiptWithOutcome: near.ReceiptWithOutcome
): void {
  if (action.kind != near.ActionKind.FUNCTION_CALL) {
    return;
  }

  const outcome = receiptWithOutcome.outcome;
  // const functionCall = action.toFunctionCall();
  // const ipfsHash = 'bafybeiew2l6admor2lx6vnfdaevuuenzgeyrpfle56yrgse4u6nnkwrfeu'
  // const methodName = functionCall.methodName

  for (let logIndex = 0; logIndex < outcome.logs.length; logIndex++) {
      const ev = parseEvent(outcome.logs[logIndex]);
      const eventDataArr = ev.get("data");
      const eventMethod = ev.get("event");

      if (!eventDataArr || !eventMethod) {
        continue;
      }

    const eventData = eventDataArr.toArray()[0];

    if (!eventData) {
      continue;
    }

      const data = eventData.toObject();
      const method = eventMethod.toString();

    if (method == "rent_add") {
      const tokenId = data.get('token_id');
      const accountId = data.get('owner_id');
      const minTime = data.get('min_time');
      const maxTime = data.get('max_time');
      const pricePerHour = data.get('price_per_hour');
      const createdAt = data.get('created_at');

      if (!tokenId || !accountId || !minTime || !maxTime || !pricePerHour || !createdAt) {
        log.error("[rent_add] - invalid args", []);
        return;
      }
      const account = Account.load(accountId.toString())
      if (!account) {
        log.error("[rent_claim] - not found account {}", [accountId.toString()]);
        return
      }

      const rent = new Rent(tokenId.toString());
      rent.tokenId = tokenId.toString();
      rent.token = tokenId.toString();
      rent.ownerId = accountId.toString();
      rent.owner = accountId.toString();
      rent.pricePerHour = pricePerHour.toString();
      rent.minTime = minTime.toU64() as i32;
      rent.maxTime = maxTime.toU64() as i32;
      rent.createdAt = createdAt.toU64() as i32;

      account.currentRents = account.currentRents + 1;

      account.save();
      rent.save();
    } else if (method == "rent_remove") {
      const tokenId = data.get('token_id');
      const accountId = data.get('account_id');

      if (!tokenId || !accountId) {
        log.error("[rent_remove] - invalid args", []);
        return;
      }

      const rent = Rent.load(tokenId.toString());

      if (!rent) {
        log.error("[rent_remove] not found rent {}", [tokenId.toString()]);
        return;
      }
      const account = Account.load(accountId.toString())
      if (!account) {
        log.error("[rent_remove] - not found account {}", [accountId.toString()]);
        return
      }

      account.currentRents = account.currentRents - 1;

      account.save();
      store.remove('Rent', tokenId.toString());
    } else if (method == "rent_pay") {
      const tokenId = data.get('token_id');
      const accountId = data.get('owner_id');
      const receiverId = data.get('receiverId');
      const time = data.get('time');
      const endTime = data.get('end_time');
      const rawPrice = data.get('price');

      if (!tokenId || !accountId || !receiverId || !time || !endTime || !rawPrice) {
        log.error("[rent_pay] - invalid args", []);
        return;
      }

      const rent = Rent.load(tokenId.toString());

      if (!rent) {
        log.error("[rent_pay] not found rent {}", [tokenId.toString()]);
        return;
      }

      const receiver = getOrCreateAccount(receiverId.toString());

      let price = BigDecimal.fromString(rawPrice.toString()).div(BigDecimal.fromString('1000000000000000000000000'));

      rent.endedAt = endTime.toU64() as i32;
      receiver.totalPayRents = receiver.totalPayRents + 1;
      receiver.totalPayRentsPrice = receiver.totalPayRentsPrice.plus(price);

      rent.save();
      receiver.save();
    } else if (method == "rent_claim") {
      const tokenId = data.get('token_id');
      const accountId = data.get('account_id');
      const renterId = data.get('renter_id');

      if (!tokenId || !accountId || !renterId) {
        log.error("[rent_claim] - invalid args", []);
        return;
      }
      const account = Account.load(accountId.toString())
      if (!account) {
        log.error("[rent_claim] - not found account {}", [accountId.toString()]);
        return
      }

      account.currentRents = account.currentRents - 1;
      store.remove('Rent', tokenId.toString());
    }
  }
}
