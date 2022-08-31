export interface SelectOption {
  label: string;
  value: string;
}

export interface TreeSelectOption extends SelectOption {
  children?: SelectOption[];
}
