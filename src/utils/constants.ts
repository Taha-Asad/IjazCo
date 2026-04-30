export const API_BASE_URL = import.meta.env.VITE_API_URL || "/api/v1";

export const ROLES = {
  ADMIN: "admin",
  INVENTORY_MANAGER: "inventory_manager",
  SALES_USER: "sales_user",
  IMPORT_CLERK: "import_clerk",
} as const;

export const INVOICE_STATUSES = [
  { value: "draft", label: "Draft" },
  { value: "approved", label: "Approved" },
  { value: "paid", label: "Paid" },
  { value: "partial", label: "Partial" },
  { value: "cancelled", label: "Cancelled" },
];

export const PO_STATUSES = [
  { value: "draft", label: "Draft" },
  { value: "submitted", label: "Submitted" },
  { value: "received", label: "Received" },
  { value: "cancelled", label: "Cancelled" },
];

export const PAYMENT_METHODS = [
  { value: "cash", label: "Cash" },
  { value: "bank_transfer", label: "Bank Transfer" },
  { value: "cheque", label: "Cheque" },
  { value: "credit_card", label: "Credit Card" },
];

export const PAGE_SIZES = [10, 20, 50, 100];

export const DATE_FORMAT = "YYYY-MM-DD";
export const DATETIME_FORMAT = "YYYY-MM-DD HH:mm";
