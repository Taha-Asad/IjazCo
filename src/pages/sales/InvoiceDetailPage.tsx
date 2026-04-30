import {
  Stack,
  Card,
  Group,
  Text,
  Badge,
  Button,
  Tabs,
  Table,
  Divider,
  SimpleGrid,
  Skeleton,
  Modal,
} from "@mantine/core";
import { useDisclosure } from "@mantine/hooks";
import { useParams, useNavigate } from "react-router-dom";
import { useQuery, useMutation, useQueryClient } from "@tanstack/react-query";
import { notifications } from "@mantine/notifications";
import { IconCheck, IconCreditCard } from "@tabler/icons-react";
import { useForm } from "@mantine/form";
import { DatePickerInput } from "@mantine/dates";
import { Select, NumberInput } from "@mantine/core";
import { PageHeader } from "../../components/common/PageHeader";
import { StatusBadge } from "../../components/common/StatusBadge";
import { StatCard } from "../../components/common/StatCard";
import { openConfirmModal } from "../../components/common/ConfirmModal";
import { salesApi } from "../../api/sales";
import {
  formatCurrency,
  formatDate,
  formatDateTime,
} from "../../utils/formatters";
import { PAYMENT_METHODS } from "../../utils/constants";
import { IconCurrencyDollar, IconReceipt } from "@tabler/icons-react";

export function InvoiceDetailPage() {
  const { id } = useParams<{ id: string }>();
  const navigate = useNavigate();
  const queryClient = useQueryClient();
  const [payOpened, { open: openPay, close: closePay }] = useDisclosure(false);

  const { data, isLoading } = useQuery({
    queryKey: ["invoice", id],
    queryFn: () => salesApi.getById(id!),
    enabled: !!id,
  });

  const { data: itemsData } = useQuery({
    queryKey: ["invoice-items", id],
    queryFn: () => salesApi.getItems(id!),
    enabled: !!id,
  });

  const approveMutation = useMutation({
    mutationFn: () => salesApi.approve(id!),
    onSuccess: () => {
      notifications.show({
        title: "Approved",
        message: "Invoice approved.",
        color: "green",
      });
      queryClient.invalidateQueries({ queryKey: ["invoice", id] });
    },
  });

  const paymentForm = useForm({
    initialValues: {
      amount: 0,
      payment_method: "cash",
      payment_date: new Date(),
      reference: "",
    },
    validate: {
      amount: (v) => (v <= 0 ? "Amount must be > 0" : null),
    },
  });

  const paymentMutation = useMutation({
    mutationFn: (v: any) =>
      salesApi.recordPayment(id!, {
        ...v,
        payment_date: v.payment_date.toISOString(),
      }),
    onSuccess: () => {
      notifications.show({
        title: "Recorded",
        message: "Payment recorded.",
        color: "green",
      });
      queryClient.invalidateQueries({ queryKey: ["invoice", id] });
      closePay();
    },
  });

  const invoice = data?.data;
  const items = itemsData?.data || [];

  if (isLoading) return <Skeleton height={400} />;
  if (!invoice) return <Text>Invoice not found.</Text>;

  return (
    <Stack>
      <PageHeader
        title={invoice.invoice_number}
        description={`Customer: ${invoice.customer_name}`}
        breadcrumbs={[
          { label: "Home", path: "/" },
          { label: "Sales", path: "/sales" },
          { label: invoice.invoice_number },
        ]}
        action={
          invoice.status === "draft"
            ? {
                label: "Approve Invoice",
                icon: <IconCheck size={16} />,
                onClick: () =>
                  openConfirmModal({
                    title: "Approve Invoice",
                    message: "This will deduct stock. Continue?",
                    onConfirm: () => approveMutation.mutate(),
                  }),
              }
            : invoice.due_amount > 0
              ? {
                  label: "Record Payment",
                  icon: <IconCreditCard size={16} />,
                  onClick: () => {
                    paymentForm.setFieldValue("amount", invoice.due_amount);
                    openPay();
                  },
                }
              : undefined
        }
      />

      <SimpleGrid cols={{ base: 1, sm: 3 }}>
        <StatCard
          title="Total Amount"
          value={formatCurrency(invoice.total_amount)}
          icon={<IconCurrencyDollar size={20} />}
          color="blue"
        />
        <StatCard
          title="Paid Amount"
          value={formatCurrency(invoice.paid_amount)}
          icon={<IconCurrencyDollar size={20} />}
          color="green"
        />
        <StatCard
          title="Balance Due"
          value={formatCurrency(invoice.due_amount)}
          icon={<IconReceipt size={20} />}
          color={invoice.due_amount > 0 ? "red" : "green"}
        />
      </SimpleGrid>

      <Card withBorder radius="md" p="lg">
        <Group justify="space-between" mb="md">
          <Group>
            <StatusBadge status={invoice.status} />
            {invoice.due_date && (
              <Text size="sm" c="dimmed">
                Due: {formatDate(invoice.due_date)}
              </Text>
            )}
          </Group>
          <Text size="xs" c="dimmed">
            Created {formatDateTime(invoice.created_at)}
          </Text>
        </Group>

        <Divider mb="md" />

        <Table striped withTableBorder withColumnBorders mb="md">
          <Table.Thead>
            <Table.Tr>
              <Table.Th>Item</Table.Th>
              <Table.Th ta="right">Qty</Table.Th>
              <Table.Th ta="right">Unit Price</Table.Th>
              <Table.Th ta="right">Discount</Table.Th>
              <Table.Th ta="right">Total</Table.Th>
            </Table.Tr>
          </Table.Thead>
          <Table.Tbody>
            {items.map((item) => (
              <Table.Tr key={item.id}>
                <Table.Td>{item.item_name}</Table.Td>
                <Table.Td ta="right">{item.quantity}</Table.Td>
                <Table.Td ta="right">
                  {formatCurrency(item.unit_price)}
                </Table.Td>
                <Table.Td ta="right">{item.discount}%</Table.Td>
                <Table.Td ta="right" fw={600}>
                  {formatCurrency(item.total)}
                </Table.Td>
              </Table.Tr>
            ))}
          </Table.Tbody>
        </Table>

        <Group justify="flex-end">
          <Stack gap="xs" w={240}>
            <Group justify="space-between">
              <Text c="dimmed" size="sm">
                Subtotal
              </Text>
              <Text size="sm">{formatCurrency(invoice.subtotal)}</Text>
            </Group>
            <Group justify="space-between">
              <Text c="dimmed" size="sm">
                Tax
              </Text>
              <Text size="sm">{formatCurrency(invoice.tax_amount)}</Text>
            </Group>
            <Group justify="space-between">
              <Text c="dimmed" size="sm">
                Discount
              </Text>
              <Text size="sm" c="red">
                -{formatCurrency(invoice.discount_amount)}
              </Text>
            </Group>
            <Divider />
            <Group justify="space-between">
              <Text fw={700}>Total</Text>
              <Text fw={700}>{formatCurrency(invoice.total_amount)}</Text>
            </Group>
          </Stack>
        </Group>

        {invoice.notes && (
          <>
            <Divider my="md" />
            <Text size="sm" c="dimmed">
              Notes: {invoice.notes}
            </Text>
          </>
        )}
      </Card>

      <Modal opened={payOpened} onClose={closePay} title="Record Payment">
        <form onSubmit={paymentForm.onSubmit((v) => paymentMutation.mutate(v))}>
          <Stack>
            <NumberInput
              label="Payment Amount"
              prefix="$"
              decimalScale={2}
              min={0.01}
              max={invoice.due_amount}
              required
              {...paymentForm.getInputProps("amount")}
            />
            <Select
              label="Payment Method"
              data={PAYMENT_METHODS}
              required
              {...paymentForm.getInputProps("payment_method")}
            />
            <DatePickerInput
              label="Payment Date"
              valueFormat="MMM DD, YYYY"
              required
              {...paymentForm.getInputProps("payment_date")}
            />
            <NumberInput
              label="Reference"
              placeholder="Cheque/transfer ref"
              {...paymentForm.getInputProps("reference")}
            />
            <Button type="submit" loading={paymentMutation.isPending}>
              Record Payment
            </Button>
          </Stack>
        </form>
      </Modal>
    </Stack>
  );
}
