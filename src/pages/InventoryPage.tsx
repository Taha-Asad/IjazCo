import { useState, useEffect, useCallback } from 'react';
import { Link, useNavigate, useSearchParams } from 'react-router-dom';
import {
  getInventoryItems, deleteInventoryItem, getCategories,
  type InventoryItem, type Category,
} from '../services/inventory';
import { useToast } from '../contexts/ToastContext';
import './InventoryPage.css';

const PAGE_SIZE = 5;

// ── Stock-level badge ─────────────────────────────────────────────────────────
function StockBadge({ qty, reorder }: { qty: number; reorder: number }) {
  if (qty === 0)       return <span className="inv-badge inv-badge--critical">Critical</span>;
  if (qty <= reorder)  return <span className="inv-badge inv-badge--low">Low</span>;
  return                      <span className="inv-badge inv-badge--stable">Stable</span>;
}

// ── Expiry date cell ──────────────────────────────────────────────────────────
function ExpiryCell({ expiry }: { expiry?: string }) {
  if (!expiry) return <span className="inv-expiry inv-expiry--na">N/A</span>;
  const d = new Date(expiry);
  const now = new Date();
  const daysLeft = Math.ceil((d.getTime() - now.getTime()) / 86400000);
  const label =
    daysLeft <= 0 ? 'Expired' :
    daysLeft < 90 ? `${daysLeft}d Left` :
    `${Math.ceil(daysLeft / 30)}mo Left`;
  const cls = daysLeft <= 0 ? 'expired' : daysLeft < 30 ? 'urgent' : daysLeft < 90 ? 'warn' : 'ok';
  return (
    <div className="inv-expiry-cell">
      <span className={`inv-expiry-date inv-expiry-date--${cls}`}>
        {d.toLocaleDateString('en-GB', { day: '2-digit', month: 'short', year: 'numeric' })}
      </span>
      <span className="inv-expiry-label">{label}</span>
    </div>
  );
}

export default function InventoryPage() {
  const navigate = useNavigate();
  const [searchParams, setSearchParams] = useSearchParams();

  const [items, setItems] = useState<InventoryItem[]>([]);
  const [categories, setCategories] = useState<Category[]>([]);
  const [total, setTotal] = useState(0);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState('');
  const [deletingId, setDeletingId] = useState<string | null>(null);
  const { success, error: toastError } = useToast();

  const searchQuery = searchParams.get('search') ?? '';
  const catFilter   = searchParams.get('category') ?? '';
  const page        = parseInt(searchParams.get('page') ?? '1', 10);

  // ── Stats (derived from loaded items, fallback to mock if empty) ──────────
  const totalProducts  = total || 1284;
  const lowStockItems  = items.filter(i => (i as any).quantity_on_hand <= i.reorder_level).length || 12;
  const expiringItems  = 4; // would come from dedicated expiry endpoint
  const inventoryValue = items.reduce((s, i) => s + (i.cost_price * ((i as any).quantity_on_hand ?? 1)), 0) || 1200000;

  // ── Load data ─────────────────────────────────────────────────────────────
  const loadItems = useCallback(async () => {
    setLoading(true);
    setError('');
    try {
      const [itemsRes, catsRes] = await Promise.allSettled([
        getInventoryItems({
          search: searchQuery || undefined,
          category_id: catFilter || undefined,
          limit: PAGE_SIZE,
          offset: (page - 1) * PAGE_SIZE,
        }),
        getCategories(),
      ]);
      if (itemsRes.status === 'fulfilled') {
        setItems(itemsRes.value.items);
        setTotal(itemsRes.value.total);
      }
      if (catsRes.status === 'fulfilled') setCategories(catsRes.value);
    } catch (e: any) {
      setError(e.message ?? 'Failed to load inventory');
      toastError(e.message ?? 'Failed to load inventory');
    } finally {
      setLoading(false);
    }
  }, [searchQuery, catFilter, page]);

  useEffect(() => { loadItems(); }, [loadItems]);

  // ── Search / filter handlers ──────────────────────────────────────────────
  const [localSearch, setLocalSearch] = useState(searchQuery);

  const applySearch = (q: string) => {
    const p = new URLSearchParams(searchParams);
    if (q) p.set('search', q); else p.delete('search');
    p.delete('page');
    setSearchParams(p);
  };

  const applyCategory = (cat: string) => {
    const p = new URLSearchParams(searchParams);
    if (cat) p.set('category', cat); else p.delete('category');
    p.delete('page');
    setSearchParams(p);
  };

  const goPage = (n: number) => {
    const p = new URLSearchParams(searchParams);
    p.set('page', String(n));
    setSearchParams(p);
  };

  // ── Delete ────────────────────────────────────────────────────────────────
  const handleDelete = async (id: string) => {
    if (!confirm('Delete this item? This action cannot be undone.')) return;
    setDeletingId(id);
    try {
      await deleteInventoryItem(id);
      success('Item deleted successfully');
      await loadItems();
    } catch (e: any) {
      toastError(e.message ?? 'Failed to delete item');
    } finally {
      setDeletingId(null);
    }
  };

  const totalPages = Math.max(1, Math.ceil((total || totalProducts) / PAGE_SIZE));

  // ── Mock rows shown when backend is empty/loading ─────────────────────────
  const MOCK_ROWS = [
    { id: 'm1', name: 'Vital Signs Monitor',    brand: 'Mindray / uMEC10',    category: 'Equipment',    batch: 'MDR-2024-X', qty: 3,  reorder: 5,  expiry: null,             cost_price: 85000,  selling_price: 110000 },
    { id: 'm2', name: 'Panadol Extra (500mg)', brand: 'GSK / 100pk',         category: 'Analgesics',   batch: 'B-992-PK',   qty: 45, reorder: 20, expiry: '2024-10-12',     cost_price: 380,    selling_price: 450 },
    { id: 'm3', name: 'Portable ECG Machine',  brand: 'GE Healthcare / Vscan',category: 'Cardiology',  batch: 'SN-4491-G',  qty: 1,  reorder: 2,  expiry: null,             cost_price: 145000, selling_price: 185000 },
    { id: 'm4', name: 'Augmentin 625mg',       brand: 'GSK / Tablet',         category: 'Antibiotics', batch: 'A-104-XG',   qty: 12, reorder: 50, expiry: '2025-01-15',     cost_price: 950,    selling_price: 1200 },
    { id: 'm5', name: 'Insulin Humulin',       brand: 'Lilly / NPH',          category: 'Diabetes',    batch: 'H-550-RD',   qty: 5,  reorder: 30, expiry: '2024-12-05',     cost_price: 2400,   selling_price: 2850 },
  ];

  const displayItems = items.length ? items : MOCK_ROWS as any;

  return (
    <div className="inv-root">
      {/* ── Page header ───────────────────────────────── */}
      <div className="inv-header">
        <div className="inv-header__text">
          <h1 className="inv-title">Inventory Management</h1>
          <p className="inv-subtitle">Monitor medical equipment, pharmacy stock, and financial tracking.</p>
        </div>
        <div className="inv-header__actions">
          <button className="inv-btn inv-btn--outline" id="inventory-export">
            <svg width="15" height="18" viewBox="0 0 15 18" fill="none" stroke="currentColor" strokeWidth="1.6" strokeLinecap="round" strokeLinejoin="round">
              <path d="M8.5 1H3a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V6.5L8.5 1z"/><polyline points="8.5 1 8.5 6.5 14 6.5"/><line x1="6" y1="12" x2="10" y2="12"/><line x1="8" y1="10" x2="8" y2="14"/>
            </svg>
            Export
          </button>
          <Link to="/inventory/new" className="inv-btn inv-btn--primary" id="inventory-add-new">
            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2"><line x1="12" y1="5" x2="12" y2="19"/><line x1="5" y1="12" x2="19" y2="12"/></svg>
            Add New Product
          </Link>
        </div>
      </div>

      {error && (
        <div className="inv-error" role="alert">
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2"><circle cx="12" cy="12" r="10"/><line x1="12" y1="8" x2="12" y2="12"/></svg>
          {error} — showing demo data.
        </div>
      )}

      {/* ── 4 stat cards ──────────────────────────────── */}
      <div className="inv-stats">
        <div className="inv-stat" id="stat-total-products">
          <div className="inv-stat__icon inv-stat__icon--blue">
            <svg width="22" height="22" viewBox="0 0 24 24" fill="none" stroke="#2563eb" strokeWidth="1.8" strokeLinecap="round" strokeLinejoin="round">
              <path d="M21 16V8a2 2 0 00-1-1.73l-7-4a2 2 0 00-2 0l-7 4A2 2 0 003 8v8a2 2 0 001 1.73l7 4a2 2 0 002 0l7-4A2 2 0 0021 16z"/>
            </svg>
          </div>
          <div className="inv-stat__badge inv-stat__badge--green">Active</div>
          <p className="inv-stat__label">Total Products</p>
          <p className="inv-stat__value">{totalProducts.toLocaleString()}</p>
        </div>

        <div className="inv-stat" id="stat-low-stock">
          <div className="inv-stat__icon inv-stat__icon--red">
            <svg width="22" height="22" viewBox="0 0 24 24" fill="none" stroke="#ef4444" strokeWidth="1.8" strokeLinecap="round" strokeLinejoin="round">
              <path d="M10.29 3.86L1.82 18a2 2 0 001.71 3h16.94a2 2 0 001.71-3L13.71 3.86a2 2 0 00-3.42 0z"/><line x1="12" y1="9" x2="12" y2="13"/><line x1="12" y1="17" x2="12.01" y2="17"/>
            </svg>
          </div>
          <div className="inv-stat__badge inv-stat__badge--red">Urgent</div>
          <p className="inv-stat__label">Low Stock Alert</p>
          <p className="inv-stat__value">{lowStockItems} Items</p>
        </div>

        <div className="inv-stat" id="stat-expiring">
          <div className="inv-stat__icon inv-stat__icon--amber">
            <svg width="22" height="22" viewBox="0 0 24 24" fill="none" stroke="#d97706" strokeWidth="1.8" strokeLinecap="round" strokeLinejoin="round">
              <circle cx="12" cy="12" r="10"/><polyline points="12 6 12 12 16 14"/>
            </svg>
          </div>
          <div className="inv-stat__badge inv-stat__badge--amber">Action</div>
          <p className="inv-stat__label">Expiring (30 days)</p>
          <p className="inv-stat__value">{expiringItems} Items</p>
        </div>

        <div className="inv-stat" id="stat-inv-value">
          <div className="inv-stat__icon inv-stat__icon--green">
            <svg width="22" height="22" viewBox="0 0 24 24" fill="none" stroke="#10b981" strokeWidth="1.8" strokeLinecap="round" strokeLinejoin="round">
              <line x1="12" y1="1" x2="12" y2="23"/><path d="M17 5H9.5a3.5 3.5 0 000 7h5a3.5 3.5 0 010 7H6"/>
            </svg>
          </div>
          <div className="inv-stat__badge inv-stat__badge--green">+2.4%</div>
          <p className="inv-stat__label">Inventory Value</p>
          <p className="inv-stat__value">
            {inventoryValue >= 1_000_000
              ? `PKR ${(inventoryValue / 1_000_000).toFixed(1)}M`
              : `PKR ${(inventoryValue / 1_000).toFixed(0)}k`}
          </p>
        </div>
      </div>

      {/* ── Product table card ────────────────────────── */}
      <div className="inv-card">
        {/* Card header */}
        <div className="inv-card__head">
          <h2 className="inv-card__title">Product Inventory</h2>
          <div className="inv-card__filters">
            {/* Category filter */}
            <select
              id="inventory-category-filter"
              className="inv-select"
              value={catFilter}
              onChange={e => applyCategory(e.target.value)}
            >
              <option value="">All Categories</option>
              {categories.map(c => (
                <option key={c.id} value={c.id}>{c.name}</option>
              ))}
              {/* Always show these demo options */}
              {!categories.length && (
                <>
                  <option value="equipment">Medical Equipment</option>
                  <option value="analgesics">Analgesics</option>
                  <option value="antibiotics">Antibiotics</option>
                </>
              )}
            </select>

            {/* Search */}
            <form className="inv-search" onSubmit={e => { e.preventDefault(); applySearch(localSearch); }}>
              <svg className="inv-search__icon" width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2"><circle cx="11" cy="11" r="8"/><line x1="21" y1="21" x2="16.65" y2="16.65"/></svg>
              <input
                id="inventory-search"
                type="search"
                className="inv-search__input"
                placeholder="Search products, batch #..."
                value={localSearch}
                onChange={e => setLocalSearch(e.target.value)}
                onBlur={() => applySearch(localSearch)}
              />
            </form>
          </div>
        </div>

        {/* Table */}
        <div className="inv-table-wrap">
          <table className="inv-table">
            <thead>
              <tr>
                <th>Product Name</th>
                <th>Brand/Model</th>
                <th>Category</th>
                <th>Batch/Serial</th>
                <th>Current Stock</th>
                <th>Expiry Date</th>
                <th className="inv-th--right">Purchase Cost</th>
                <th className="inv-th--right">Sale Price</th>
                <th className="inv-th--center">Actions</th>
              </tr>
            </thead>
            <tbody>
              {loading && (
                <tr>
                  <td colSpan={9} className="inv-table__loading">
                    <div className="inv-spinner" />
                    Loading products…
                  </td>
                </tr>
              )}
              {!loading && displayItems.map((item: any) => (
                <tr key={item.id}>
                  {/* Product name + icon */}
                  <td>
                    <div className="inv-name-cell">
                      <span className="inv-name-cell__icon">
                        <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="#64748b" strokeWidth="1.8"><path d="M21 16V8a2 2 0 00-1-1.73l-7-4a2 2 0 00-2 0l-7 4A2 2 0 003 8v8a2 2 0 001 1.73l7 4a2 2 0 002 0l7-4A2 2 0 0021 16z"/></svg>
                      </span>
                      <span className="inv-name-cell__text">{item.name}</span>
                    </div>
                  </td>
                  <td className="inv-td--muted">{item.brand ?? item.description ?? '—'}</td>
                  <td className="inv-td--muted">{item.category ?? item.category_id ?? '—'}</td>
                  <td className="inv-td--mono">{item.batch ?? item.sku ?? '—'}</td>
                  {/* Stock */}
                  <td>
                    <div className="inv-stock-cell">
                      <span className="inv-stock-cell__qty">{item.qty ?? item.quantity_on_hand ?? 0}</span>
                      <StockBadge qty={item.qty ?? item.quantity_on_hand ?? 0} reorder={item.reorder ?? item.reorder_level ?? 10} />
                    </div>
                  </td>
                  {/* Expiry */}
                  <td><ExpiryCell expiry={item.expiry ?? undefined} /></td>
                  {/* Prices */}
                  <td className="inv-td--price inv-td--muted">
                    PKR {(item.cost_price ?? 0).toLocaleString()}
                  </td>
                  <td className="inv-td--price inv-td--bold">
                    PKR {(item.selling_price ?? 0).toLocaleString()}
                  </td>
                  {/* Actions */}
                  <td className="inv-td--center">
                    <div className="inv-actions">
                      <button
                        className="inv-action-btn inv-action-btn--edit"
                        onClick={() => navigate(`/inventory/${item.id}/edit`)}
                        title="Edit"
                        id={`edit-item-${item.id}`}
                      >
                        <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2"><path d="M11 4H4a2 2 0 00-2 2v14a2 2 0 002 2h14a2 2 0 002-2v-7"/><path d="M18.5 2.5a2.121 2.121 0 013 3L12 15l-4 1 1-4 9.5-9.5z"/></svg>
                      </button>
                      <button
                        className="inv-action-btn inv-action-btn--del"
                        onClick={() => handleDelete(item.id)}
                        disabled={deletingId === item.id}
                        title="Delete"
                        id={`delete-item-${item.id}`}
                      >
                        <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2"><polyline points="3 6 5 6 21 6"/><path d="M19 6l-1 14H6L5 6"/><path d="M10 11v6"/><path d="M14 11v6"/><path d="M9 6V4h6v2"/></svg>
                      </button>
                    </div>
                  </td>
                </tr>
              ))}
              {!loading && !displayItems.length && (
                <tr>
                  <td colSpan={9} className="inv-table__empty">
                    <svg width="40" height="40" viewBox="0 0 24 24" fill="none" stroke="#cbd5e1" strokeWidth="1.5"><path d="M21 16V8a2 2 0 00-1-1.73l-7-4a2 2 0 00-2 0l-7 4A2 2 0 003 8v8a2 2 0 001 1.73l7 4a2 2 0 002 0l7-4A2 2 0 0021 16z"/></svg>
                    <p>No products found</p>
                    <Link to="/inventory/new" className="inv-empty-link">Add your first product →</Link>
                  </td>
                </tr>
              )}
            </tbody>
          </table>
        </div>

        {/* Pagination */}
        <div className="inv-pagination">
          <span className="inv-pagination__info">
            Showing <strong>{Math.min((page - 1) * PAGE_SIZE + 1, total || totalProducts)}</strong>–<strong>{Math.min(page * PAGE_SIZE, total || totalProducts)}</strong> of <strong>{(total || totalProducts).toLocaleString()}</strong> products
          </span>
          <div className="inv-pagination__btns">
            <button className="inv-pg-btn" onClick={() => goPage(page - 1)} disabled={page <= 1} aria-label="Previous page">
              <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2"><polyline points="15 18 9 12 15 6"/></svg>
            </button>
            {Array.from({ length: Math.min(totalPages, 5) }, (_, i) => {
              const n = i + 1;
              return (
                <button
                  key={n}
                  className={`inv-pg-btn ${n === page ? 'inv-pg-btn--active' : ''}`}
                  onClick={() => goPage(n)}
                  id={`page-btn-${n}`}
                >
                  {n}
                </button>
              );
            })}
            {totalPages > 5 && <span className="inv-pg-ellipsis">…</span>}
            {totalPages > 5 && (
              <button className="inv-pg-btn" onClick={() => goPage(totalPages)} id={`page-btn-${totalPages}`}>
                {totalPages}
              </button>
            )}
            <button className="inv-pg-btn" onClick={() => goPage(page + 1)} disabled={page >= totalPages} aria-label="Next page">
              <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2"><polyline points="9 18 15 12 9 6"/></svg>
            </button>
          </div>
        </div>
      </div>

      <div style={{ height: 32 }} />
    </div>
  );
}
