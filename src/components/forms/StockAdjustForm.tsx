import { NumberInput, TextInput, Button, Stack } from "@mantine/core";
import { useForm } from "@mantine/form";

interface StockAdjustFormProps {
  itemId: string;
  branchId: string;
  currentQuantity: number;
  onSubmit: (values: any) => Promise<void>;
  loading?: boolean;
}

export function StockAdjustForm({
  currentQuantity,
  onSubmit,
  loading,
}: StockAdjustFormProps) {
  const form = useForm({
    initialValues: {
      quantity: 0,
      reason: "",
      notes: "",
    },
    validate: {
      quantity: (v) => (v === 0 ? "Quantity required" : null),
      reason: (v) => (!v ? "Reason required" : null),
    },
  });

  return (
    <form onSubmit={form.onSubmit(onSubmit)}>
      <Stack>
        <NumberInput
          label="Quantity Adjustment"
          placeholder="Enter quantity"
          required
          {...form.getInputProps("quantity")}
        />
        <TextInput
          label="Reason"
          placeholder="e.g. Damaged goods, Found inventory"
          required
          {...form.getInputProps("reason")}
        />
        <TextInput
          label="Notes"
          placeholder="Additional notes (optional)"
          {...form.getInputProps("notes")}
        />
        <Button type="submit" loading={loading}>
          Adjust Stock
        </Button>
      </Stack>
    </form>
  );
}
