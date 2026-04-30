import {
  Select,
  TextInput,
  NumberInput,
  Textarea,
  Button,
  Stack,
  Group,
  SimpleGrid,
  Text,
} from "@mantine/core";
import { DatePickerInput } from "@mantine/dates";
import { useForm } from "@mantine/form";
import { useQuery } from "@tanstack/react-query";
import { suppliersApi } from "../../api/suppliers";

const SHIPPING_METHODS = [
  { value: "sea", label: "Sea Freight" },
  { value: "air", label: "Air Freight" },
  { value: "road", label: "Road Transport" },
  { value: "courier", label: "Courier" },
];

interface ImportOrderFormProps {
  initialValues?: any;
  onSubmit: (values: any) => Promise<void>;
  loading?: boolean;
}

export function ImportOrderForm({
  initialValues,
  onSubmit,
  loading,
}: ImportOrderFormProps) {
  const { data: suppliersData } = useQuery({
    queryKey: ["suppliers-select"],
    queryFn: () => suppliersApi.list({ per_page: 200 }),
  });

  const supplierOptions =
    suppliersData?.data?.map((s) => ({
      value: s.id,
      label: s.name,
    })) || [];

  const form = useForm({
    initialValues: {
      supplier_id: initialValues?.supplier_id || "",
      origin_country: initialValues?.origin_country || "",
      shipping_method: initialValues?.shipping_method || "",
      tracking_number: initialValues?.tracking_number || "",
      shipping_cost: initialValues?.shipping_cost || 0,
      customs_duty: initialValues?.customs_duty || 0,
      other_charges: initialValues?.other_charges || 0,
      estimated_arrival: initialValues?.estimated_arrival
        ? new Date(initialValues.estimated_arrival)
        : (null as Date | null),
      notes: initialValues?.notes || "",
    },
    validate: {
      supplier_id: (v) => (!v ? "Supplier required" : null),
      origin_country: (v) => (!v ? "Origin country required" : null),
    },
  });

  const totalCost =
    form.values.shipping_cost +
    form.values.customs_duty +
    form.values.other_charges;

  return (
    <form onSubmit={form.onSubmit(onSubmit)}>
      <Stack>
        <Select
          label="Supplier"
          placeholder="Select supplier"
          data={supplierOptions}
          required
          searchable
          {...form.getInputProps("supplier_id")}
        />
        <SimpleGrid cols={2}>
          <TextInput
            label="Origin Country"
            placeholder="e.g. Germany"
            required
            {...form.getInputProps("origin_country")}
          />
          <Select
            label="Shipping Method"
            data={SHIPPING_METHODS}
            clearable
            {...form.getInputProps("shipping_method")}
          />
        </SimpleGrid>
        <SimpleGrid cols={2}>
          <TextInput
            label="Tracking Number"
            placeholder="e.g. TRACK123456"
            {...form.getInputProps("tracking_number")}
          />
          <DatePickerInput
            label="Estimated Arrival"
            valueFormat="MMM DD, YYYY"
            {...form.getInputProps("estimated_arrival")}
          />
        </SimpleGrid>
        <SimpleGrid cols={3}>
          <NumberInput
            label="Shipping Cost"
            prefix="$"
            decimalScale={2}
            min={0}
            {...form.getInputProps("shipping_cost")}
          />
          <NumberInput
            label="Customs Duty"
            prefix="$"
            decimalScale={2}
            min={0}
            {...form.getInputProps("customs_duty")}
          />
          <NumberInput
            label="Other Charges"
            prefix="$"
            decimalScale={2}
            min={0}
            {...form.getInputProps("other_charges")}
          />
        </SimpleGrid>

        <Group justify="flex-end">
          <Text fw={700}>Total Cost: ${totalCost.toFixed(2)}</Text>
        </Group>

        <Textarea label="Notes" rows={3} {...form.getInputProps("notes")} />
        <Button type="submit" loading={loading}>
          {initialValues ? "Save Changes" : "Create Import Order"}
        </Button>
      </Stack>
    </form>
  );
}
