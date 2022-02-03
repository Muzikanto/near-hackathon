import { near, store } from "@graphprotocol/graph-ts"
import { log } from '@graphprotocol/graph-ts'
import { Sale, SaleCondition, Account } from '../generated/schema';
import { parseEvent } from "./utils";
import { getOrCreateAccount } from "./helpers/account";

export function handleMarket(
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

    if (method == "market_create_sale") {
      const saleStr = data.get('sale');

      if (!saleStr) {
        log.error("[market_create_sale] - invalid args", []);
        return;
      }

      const saleObj = saleStr.toObject();
      const ownerId = saleObj.get('owner_id');
      const nftContractId = saleObj.get('nft_contract_id');
      const tokenId = saleObj.get('token_id');
      const saleConditions = saleObj.get('sale_conditions');
      const createdAt = saleObj.get('created_at');
      const isAuction = saleObj.get('is_auction');

      if (!ownerId || !ownerId || !nftContractId || !tokenId || !saleConditions || !createdAt) {
        log.error("[market_create_sale] - invalid args", []);
        return;
      }

      const account = Account.load(ownerId.toString());

      if (!account) {
        log.error("[market_create_sale] - not found account {}", [ownerId.toString()]);
        return;
      }

      const saleId = nftContractId.toString() + "||" + tokenId.toString();

      const sale = new Sale(saleId);

      sale.tokenId = tokenId.toString();
      sale.token = tokenId.toString();
      sale.ownerId = ownerId.toString();
      sale.owner = ownerId.toString();
      sale.nftContractId = nftContractId.toString();
      sale.createdAt = createdAt.toU64() as i32;
      sale.isAuction = isAuction ? isAuction.toBool() : false;

      const saleConditionsObj = saleConditions.toObject();

      for(let i = 0; i < saleConditionsObj.entries.length; i ++ ) {
        const conditionRaw = saleConditionsObj.entries[i];

        const saleConditionId = saleId + "||" + conditionRaw.key.toString();

        const saleCondition = new SaleCondition(saleConditionId);
        saleCondition.saleId = saleId.toString();
        saleCondition.sale = saleId.toString();
        saleCondition.ftTokenId = conditionRaw.key;
        saleCondition.price = conditionRaw.value.toString();

        saleCondition.save();
      }

      account.currentSales = account.currentSales + 1;

      account.save();
      sale.save();
    } else if (method == "market_update_sale") {
      const tokenId = data.get('token_id');
      const ownerId = data.get('owner_id');
      const nftContractId = data.get('nft_contract_id');
      const ftTokenId = data.get('ft_token_id');
      const price = data.get('price');

      if (!tokenId || !ownerId || !nftContractId || !ftTokenId || !price) {
        log.error("[market_update_sale] - invalid args", []);
        return;
      }

      const saleId = nftContractId.toString() + "||" + tokenId.toString();
      const saleConditionId = saleId + "||" + ftTokenId.toString();

      let saleCondition = SaleCondition.load(saleConditionId);

      if (!saleCondition) {
        saleCondition = new SaleCondition(saleConditionId);
        saleCondition.saleId = saleId;
        saleCondition.sale = saleId;
      }

      saleCondition.ftTokenId = ftTokenId.toString();
      saleCondition.price = price.toString();

      saleCondition.save();
    } else if (method == "market_remove_sale") {
      const tokenId = data.get('token_id');
      const ownerId = data.get('owner_id');
      const nftContractId = data.get('nft_contract_id');

      if (!tokenId || !ownerId || !nftContractId) {
        log.error("[market_remove_sale] - invalid args", []);
        return;
      }

      const saleId = nftContractId.toString() + "||" + tokenId.toString();

      const account = Account.load(ownerId.toString());

      if (!account) {
        log.error("[market_remove_sale] - not found account {}", [ownerId.toString()]);
        return
      }

      account.currentSales = account.currentSales - 1;

      account.save();
      store.remove('Sale', saleId.toString());
    } else if (method == "market_offer") {
      const tokenId = data.get('token_id');
      const accountId = data.get('owner_id');
      const receiverId = data.get('receiver_id');
      const nftContractId = data.get('nft_contract_id');
      const payout = data.get('payout');
      const ftTokenId = data.get('ft_token_id');
      const price = data.get('price');

      if (!tokenId || !accountId || !receiverId || !nftContractId || !payout || !ftTokenId || !price) {
        log.error("[market_offer] - invalid args", []);
        return;
      }

      const saleId = nftContractId.toString() + "||" + tokenId.toString();

      const account = getOrCreateAccount(accountId.toString());
      account.currentSales = account.currentSales - 1;

      account.save();
      store.remove('Sale', saleId.toString());
    }
  }
}
