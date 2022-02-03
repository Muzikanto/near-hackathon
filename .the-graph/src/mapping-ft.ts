import { near, BigInt } from "@graphprotocol/graph-ts"
import { log } from '@graphprotocol/graph-ts'
import { parseEvent } from "./utils";
import { getOrCreateAccount } from "./helpers/account";

export function handleFt(
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

    if (method == "ft_transfer") {
      const amount = data.get('amount');
      const old_owner_id = data.get('old_owner_id');
      const new_owner_id = data.get('new_owner_id');
      const memo = data.get('memo');

      if (!amount || !new_owner_id || !old_owner_id) {
        log.error("[ft_transfer] - invalid args", []);
        return;
      }

      const sender = getOrCreateAccount(old_owner_id.toString());
      const receiver = getOrCreateAccount(new_owner_id.toString());

      sender.balance = sender.balance.minus(BigInt.fromString(amount.toString()));
      receiver.balance = receiver.balance.plus(BigInt.fromString(amount.toString()));

      if (sender.balance.lt(BigInt.zero())) {
        log.error("[ft_transfer] - zero transfer {} {} {}", [old_owner_id.toString(), new_owner_id.toString(), amount.toString()]);

        return;
      }

      sender.save();
      receiver.save();
    } else if (method == "ft_mint") {
      const amount = data.get('amount');
      const account_id = data.get('owner_id');
      const memo = data.get('memo');

      if (!account_id || !amount) {
        log.error("[ft_mint] - invalid args", []);
        return;
      }

      const account = getOrCreateAccount(account_id.toString());

      account.balance = account.balance.plus(BigInt.fromString(amount.toString()));

      account.save();
    }
  }
}
