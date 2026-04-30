import { useState, useEffect, useCallback } from 'react';
import { Link } from 'react-router-dom';
import { getDashboardStats, type DashboardStats } from '../services/dashboard';
import { getSalesInvoices, type SalesInvoice } from '../services/sales';
import { getLowStockAlerts } from '../services/stock';
import './DashboardPage.css';

// ── Tiny helper ───────────────────────────────────────────────────────────────
const fmtPKR = (n: number) =>
  n >= 1_000_000
    ? `PKR ${(n / 1_000_000).toFixed(2)}M`
    : n >= 1_000
    ? `PKR ${(n / 1_000).toFixed(0)}k`
    : `PKR ${n.toLocaleString()}`;

const STATUS_COLORS: Record<string, { bg: string; color: string; label: string }> = {
  completed: { bg: '#d1fae5', color: '#059669', label: 'Completed' },
  paid:       { bg: '#d1fae5', color: '#059669', label: 'Completed' },
  approved:   { bg: '#d1fae5', color: '#059669', label: 'Approved' },
  pending:    { bg: '#fef3c7', color: '#d97706', label: 'Pending' },
  draft:      { bg: '#f1f5f9', color: '#64748b', label: 'Draft' },
  cancelled:  { bg: '#fee2e2', color: '#ef4444', label: 'Cancelled' },
};

// ── Expiry mock data (would come from backend expiry endpoint) ────────────────
const EXPIRY_ITEMS = [
  { name: 'Panadol Extra',    batch: 'B-992', days: 12,  qty: 45,  urgency: 'red' },
  { name: 'Augmentin 625mg',  batch: 'A-104', days: 18,  qty: 12,  urgency: 'red' },
  { name: 'Brufen 400mg',     batch: 'C-221', days: 45,  qty: 200, urgency: 'amber' },
  { name: 'Insulin Humulin',  batch: 'H-550', days: 58,  qty: 5,   urgency: 'amber' },
  { name: 'Calcium Sandoz',   batch: 'S-990', months: 3, qty: 30,  urgency: 'ok' },
  { name: 'Disprin Soluble',  batch: 'D-121', months: 5, qty: 500, urgency: 'ok' },
  { name: 'Ventolin Inhaler', batch: 'V-332', months: 8, qty: 15,  urgency: 'ok' },
];

// ── Static fallback stats (shown while loading / backend not available) ───────
const FALLBACK_STATS: DashboardStats = {
  overview: {
    total_revenue: 250000, revenue_change_percent: 5,
    total_orders: 82, orders_change_percent: 8.3,
    total_customers: 1256, new_customers: 89, inventory_value: 1200000,
  },
  sales: {
    total_sales: 250000, invoice_count: 82,
    average_order_value: 3048, outstanding_amount: 25000,
    top_items: [], by_status: { draft: 2, pending: 5, approved: 60, paid: 15, cancelled: 0 },
  },
  inventory: {
    total_items: 456, total_quantity: 12560, total_cost_value: 892430,
    total_selling_value: 1234560, low_stock_items: 12, out_of_stock_items: 3,
    by_category: [],
  },
  purchases: { total_purchases: 234500, po_count: 89, pending_pos: 12, average_po_value: 2634, top_suppliers: [] },
  recent_activities: [],
  low_stock_count: 12,
  pending_approvals: { sales_invoices: 5, purchase_orders: 3 },
};

export default function DashboardPage() {
  const [stats, setStats] = useState<DashboardStats>(FALLBACK_STATS);
  const [invoices, setInvoices] = useState<SalesInvoice[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState('');
  const [expiryItems, setExpiryItems] = useState(EXPIRY_ITEMS);

  const loadData = useCallback(async () => {
    setLoading(true);
    setError('');
    try {
      const [s, inv, lowStock] = await Promise.allSettled([
        getDashboardStats(),
        getSalesInvoices({ limit: 5 }),
        getLowStockAlerts(),
      ]);
      if (s.status === 'fulfilled') setStats(s.value);
      if (inv.status === 'fulfilled') setInvoices(inv.value.invoices ?? []);
      if (lowStock.status === 'fulfilled' && Array.isArray(lowStock.value) && lowStock.value.length > 0) {
        const mapped = lowStock.value.slice(0, 8).map((item: any) => ({
          name: item.item_name || item.name || 'Unnamed Item',
          batch: item.batch_no || item.sku || 'N/A',
          days: Number(item.days_to_expiry || item.days || 30),
          qty: Number(item.quantity_on_hand || item.quantity || 0),
          urgency: Number(item.days_to_expiry || item.days || 30) <= 20 ? 'red' : Number(item.days_to_expiry || item.days || 30) <= 60 ? 'amber' : 'ok',
        }));
        setExpiryItems(mapped as any);
      }
    } catch (e: any) {
      setError(e.message ?? 'Failed to load dashboard');
    } finally {
      setLoading(false);
    }
  }, []);

  useEffect(() => { loadData(); }, [loadData]);

  const totalStockValue = stats.overview.inventory_value;
  const monthlySalesGoal = 500000;
  const salesPercent = Math.min(100, Math.round((stats.sales.total_sales / monthlySalesGoal) * 100));

  // Display invoices – use real data or show mock rows
  const displayInvoices: (SalesInvoice & { customerName?: string })[] = invoices.length
    ? invoices
    : [
        { id: '1', company_id: '', branch_id: '', customer_id: 'walk-in', invoice_number: '#ORD-2992', invoice_date: new Date().toISOString(), status: 'paid', subtotal: 4200, tax_amount: 0, discount_amount: 0, total_amount: 4200, balance_due: 0, created_at: new Date().toISOString(), customerName: 'Walk-in Customer' },
        { id: '2', company_id: '', branch_id: '', customer_id: 'c2', invoice_number: '#ORD-2991', invoice_date: new Date().toISOString(), status: 'pending', subtotal: 12500, tax_amount: 0, discount_amount: 0, total_amount: 12500, balance_due: 12500, created_at: new Date().toISOString(), customerName: 'Dr. Ahmed Clinic' },
        { id: '3', company_id: '', branch_id: '', customer_id: 'c3', invoice_number: '#ORD-2990', invoice_date: new Date().toISOString(), status: 'paid', subtotal: 8150, tax_amount: 0, discount_amount: 0, total_amount: 8150, balance_due: 0, created_at: new Date().toISOString(), customerName: 'Pharmacy B' },
      ] as any;

  return (
    <div className="db-root">
      {/* ── Page Title ─────────────────────────────────── */}
      <div className="db-titlebar">
        <div>
          <h1 className="db-title">Overview</h1>
          <p className="db-subtitle">Here's what's happening in your store today.</p>
        </div>
        <div className="db-titlebar__actions">
          <button className="db-btn db-btn--ghost" onClick={loadData} disabled={loading} id="dashboard-refresh">
            <svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" className={loading ? 'db-spin' : ''}>
              <polyline points="1 4 1 10 7 10"/><path d="M3.51 15a9 9 0 102.13-9.36L1 10"/>
            </svg>
            Refresh
          </button>
          <Link to="/sales/new" className="db-btn db-btn--primary" id="dashboard-new-sale">
            <svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2"><line x1="12" y1="5" x2="12" y2="19"/><line x1="5" y1="12" x2="19" y2="12"/></svg>
            New Sale
          </Link>
        </div>
      </div>

      {error && (
        <div className="db-error" role="alert">
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2"><circle cx="12" cy="12" r="10"/><line x1="12" y1="8" x2="12" y2="12"/><line x1="12" y1="16" x2="12.01" y2="16"/></svg>
          {error} — showing cached data.
        </div>
      )}

      {/* ── Stat Cards (3) ────────────────────────────── */}
      <div className="db-stats-row">
        {/* Total Stock Value */}
        <div className="db-stat-card" id="stat-stock-value">
          <div className="db-stat-card__icon db-stat-card__icon--blue">
            <svg width="22" height="26" viewBox="0 0 24 28" fill="none" stroke="#2563eb" strokeWidth="1.8" strokeLinecap="round" strokeLinejoin="round">
              <rect x="2" y="3" width="20" height="14" rx="2"/><path d="M16 21H8"/><path d="M12 17v6"/>
            </svg>
          </div>
          <div className="db-stat-card__badge db-stat-card__badge--green">
            <svg width="11" height="11" viewBox="0 0 12 12" fill="none" stroke="currentColor" strokeWidth="1.8"><polyline points="2 9 6 5 10 9"/></svg>
            +{stats.overview.revenue_change_percent}%
          </div>
          <p className="db-stat-card__label">Total Stock Value</p>
          <p className="db-stat-card__value">{fmtPKR(totalStockValue)}</p>
        </div>

        {/* Monthly Goal */}
        <div className="db-stat-card" id="stat-monthly-goal">
          <div className="db-stat-card__icon db-stat-card__icon--purple">
            <svg width="22" height="22" viewBox="0 0 24 24" fill="none" stroke="#9333ea" strokeWidth="1.8" strokeLinecap="round" strokeLinejoin="round">
              <circle cx="12" cy="12" r="10"/><polyline points="12 6 12 12 16 14"/>
            </svg>
          </div>
          <p className="db-stat-card__label db-stat-card__label--sm">Monthly Goal</p>
          <p className="db-stat-card__label">Sales vs Target</p>
          <p className="db-stat-card__big">{salesPercent}%</p>
          {/* Progress bar */}
          <div className="db-progress">
            <div className="db-progress__bar" style={{ width: `${salesPercent}%` }} />
          </div>
          <p className="db-stat-card__hint">Target: {fmtPKR(monthlySalesGoal)}</p>
        </div>

        {/* Outstanding Credit */}
        <div className="db-stat-card" id="stat-outstanding">
          <div className="db-stat-card__icon db-stat-card__icon--amber">
            <svg width="22" height="22" viewBox="0 0 24 24" fill="none" stroke="#d97706" strokeWidth="1.8" strokeLinecap="round" strokeLinejoin="round">
              <circle cx="12" cy="12" r="10"/><line x1="12" y1="8" x2="12" y2="12"/><line x1="12" y1="16" x2="12.01" y2="16"/>
            </svg>
          </div>
          <div className="db-stat-card__badge db-stat-card__badge--amber">
            <svg width="11" height="11" viewBox="0 0 12 12" fill="none" stroke="currentColor" strokeWidth="1.8"><polyline points="2 3 6 7 10 3"/></svg>
            Action
          </div>
          <p className="db-stat-card__label">Outstanding Credit</p>
          <p className="db-stat-card__value">{fmtPKR(stats.sales.outstanding_amount || 25000)}</p>
        </div>
      </div>

      {/* ── Bottom two panels ─────────────────────────── */}
      <div className="db-panels">
        {/* Recent Transactions */}
        <div className="db-panel db-panel--wide" id="recent-transactions">
          <div className="db-panel__head">
            <h2 className="db-panel__title">Recent Transactions</h2>
            <Link to="/sales" className="db-link" id="view-all-sales">View All</Link>
          </div>
          <div className="db-table-wrap">
            <table className="db-table">
              <thead>
                <tr>
                  <th>Order ID</th>
                  <th>Customer</th>
                  <th>Amount</th>
                  <th>Status</th>
                </tr>
              </thead>
              <tbody>
                {displayInvoices.map((inv) => {
                  const sc = STATUS_COLORS[inv.status] ?? STATUS_COLORS.draft;
                  return (
                    <tr key={inv.id}>
                      <td className="db-table__id">{inv.invoice_number}</td>
                      <td className="db-table__customer">{(inv as any).customerName ?? `Customer ${inv.customer_id.slice(0, 6)}`}</td>
                      <td className="db-table__amount">PKR {inv.total_amount.toLocaleString()}</td>
                      <td>
                        <span className="db-badge" style={{ background: sc.bg, color: sc.color }}>
                          {sc.label}
                        </span>
                      </td>
                    </tr>
                  );
                })}
              </tbody>
            </table>
          </div>
        </div>

        {/* Expiry Monitor */}
        <div className="db-panel db-panel--expiry" id="expiry-monitor">
          {/* Panel header */}
          <div className="db-expiry-head">
            <div className="db-expiry-head__left">
              <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="#313740" strokeWidth="1.8" strokeLinecap="round" strokeLinejoin="round">
                <circle cx="12" cy="12" r="10"/><polyline points="12 6 12 12 16 14"/>
              </svg>
              <span className="db-expiry-head__title">Expiry Monitor</span>
            </div>
            <span className="db-expiry-head__range">Next 365 Days</span>
            <button className="db-expiry-head__filter" aria-label="Filter">
              <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2"><line x1="4" y1="6" x2="20" y2="6"/><line x1="8" y1="12" x2="16" y2="12"/><line x1="11" y1="18" x2="13" y2="18"/></svg>
            </button>
          </div>

          {/* Expiry list */}
          <div className="db-expiry-list">
            {expiryItems.map((item, i) => (
              <div key={i} className={`db-expiry-item db-expiry-item--${item.urgency}`}>
                <div className="db-expiry-item__info">
                  <span className="db-expiry-item__name">{item.name}</span>
                  <span className="db-expiry-item__batch">Batch <strong>{item.batch}</strong></span>
                </div>
                <div className="db-expiry-item__meta">
                  <span className="db-expiry-item__days">
                    {'days' in item ? `${item.days} Days` : `${item.months} Mos`}
                  </span>
                  <span className="db-expiry-item__qty">Qty: {item.qty}</span>
                </div>
              </div>
            ))}
          </div>

          {/* Panel footer */}
          <div className="db-expiry-footer">
            <Link to="/inventory" className="db-link" id="view-expiry-report">View Full Expiry Report</Link>
            <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="#2280c3" strokeWidth="2"><polyline points="9 18 15 12 9 6"/></svg>
          </div>

          {/* Bottom link */}
          <Link to="/inventory" className="db-expiry-cta">View Full Expiry Report →</Link>
        </div>
      </div>

      {/* Spacer */}
      <div style={{ height: 32 }} />
    </div>
  );
}
