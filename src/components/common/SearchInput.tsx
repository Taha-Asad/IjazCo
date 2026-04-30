import { TextInput, TextInputProps, CloseButton } from "@mantine/core";
import { IconSearch } from "@tabler/icons-react";

interface SearchInputProps extends Omit<TextInputProps, "onChange"> {
  value: string;
  onChange: (value: string) => void;
  onClear?: () => void;
}

export function SearchInput({
  value,
  onChange,
  onClear,
  placeholder = "Search...",
  ...props
}: SearchInputProps) {
  return (
    <TextInput
      value={value}
      onChange={(e) => onChange(e.currentTarget.value)}
      placeholder={placeholder}
      leftSection={<IconSearch size={16} />}
      rightSection={
        value ? (
          <CloseButton
            size="sm"
            onClick={() => {
              onChange("");
              onClear?.();
            }}
          />
        ) : null
      }
      {...props}
    />
  );
}
