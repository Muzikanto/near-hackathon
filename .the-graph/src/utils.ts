import { json, JSONValue, log, TypedMap } from "@graphprotocol/graph-ts";

export function parseEvent(logData: string): TypedMap<string, JSONValue> {
  let outcomeLog = logData.toString();

  if (!outcomeLog.includes('EVENT_JSON:')) {
    log.info('outcomeLog skip {}', [outcomeLog]);

    return new TypedMap<string, JSONValue>();
  }

  log.info('outcomeLog {}', [outcomeLog]);

  let parsed = outcomeLog.replace('EVENT_JSON:', '');

  let jsonData = json.try_fromString(parsed);
  const jsonObject = jsonData.value.toObject();

  return jsonObject;
}

