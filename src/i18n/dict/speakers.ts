const ru = {
  mergeTitle: "Нажмите, чтобы объединить этого спикера с другим",
  mergeWith: "Объединить с…",
  reassignTitle: "Переназначить эту реплику другому спикеру",
  moveTo: "Переместить к…",
};

type T = typeof ru;

const en: T = {
  mergeTitle: "Click to merge this speaker into another",
  mergeWith: "Merge into…",
  reassignTitle: "Reassign this line to another speaker",
  moveTo: "Move to…",
};

const zh: T = {
  mergeTitle: "点击将此说话人与另一位合并", mergeWith: "合并到…", reassignTitle: "将此发言分配给另一位说话人", moveTo: "移动到…",
};

export const speakers = { ru, en, zh };
