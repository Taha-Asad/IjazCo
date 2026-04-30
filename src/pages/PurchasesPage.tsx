import { useState, useEffect } from 'react';
import { Link, useNavigate } from 'react-router-dom';
import { getPurchaseOrders, deletePurchaseOrder, PurchaseOrder } from '../services/purchases';
import './PurchasesPage.css';

export default function PurchasesPage() {
  const navigate = useNavigate();
  const [purchases, setPurchases] = useState<PurchaseOrder[]>([]);
  const [loading, setLoading] = useState(true);
  const [search, setSearch] = useState('');
  const [statusFilter, setStatusFilter] = useState('all');
  const [deletingId, setDeletingId] = useState<string | null>(null);
  const [error, setError] = useState('');

  const loadPurchases = async () => {
    try {
      setLoading(true);
      setError('');
      const data = await getPurchaseOrders({ limit: 100 });
      // API might return array directly or wrapped in { purchases }
      const purchaseList = Array.isArray(data) ? data : (data.purchases || []);
      setPurchases(purchaseList);
    } catch (err: any) {
      setError(err.message || 'Failed to load purchase orders');
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    loadPurchases();
  }, []);

  const handleDelete = async (id: string) => {
    if (!confirm('Are you sure you want to delete this purchase order?')) return;
    try {
      setDeletingId(id);
      await deletePurchaseOrder(id);
      setPurchases(prev => prev.filter(po => po.id !== id));
    } catch (err: any) {
      alert(err.message || 'Failed to delete purchase order');
    } finally {
      setDeletingId(null);
    }
  };

  const filteredPurchases = purchases.filter(po => {
    const matchesSearch = po.po_number && po.po_number.toLowerCase().includes(search.toLowerCase());
    const matchesStatus = statusFilter === 'all' || po.status === statusFilter;
    return matchesSearch && matchesStatus;
  });

  return (
    <div className="purchases-root">
      {/* ── Page Header ── */}
      <div className="purchases-header">
        <div className="purchases-header__text">
          <h1 className="purchases-title">Purchase Orders</h1>
          <p className="purchases-subtitle">Procurement from suppliers and goods receipt.</p>
        </div>
        <div className="purchases-header__actions">
          <Link to="/purchases/new" className="purchases-btn purchases-btn--primary">
            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2"><line x1="12" y1="5" x2="12" y2="19"/><line x1="5" y1="12" x2="19" y2="12"/></svg>
            Create PO
          </Link>
        </div>
      </div>

      {error && (
        <div className="purchases-error" role="alert">
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2"><circle cx="12" cy="12" r="10"/><line x1="12" y1="8" x2="12" y2="12"/></svg>
          {error}
        </div>
      )}

      {/* ── Main Card ── */}
      <div className="purchases-card">
        {/* Card Header (Search & Filter) */}
        <div className="purchases-card__head">
          <h2 className="purchases-card__title">PO Directory</h2>
          
          <div className="purchases-filters">
            <div className="purchases-search">
              <svg className="purchases-search__icon" width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2"><circle cx="11" cy="11" r="8"/><line x1="21" y1="21" x2="16.65" y2="16.65"/></svg>
              <input
                type="search"
                className="purchases-search__input"
                placeholder="Search PO #..."
                value={search}
                onChange={(e) => setSearch(e.target.value)}
              />
            </div>
            
            <select 
              className="purchases-select"
              value={statusFilter}
              onChange={(e) => setStatusFilter(e.target.value)}
            >
              <option value="all">All Statuses</option>
              <option value="draft">Draft</option>
              <option value="submitted">Submitted</option>
              <option value="confirmed">Confirmed</option>
              <option value="shipped">Shipped</option>
              <option value="received">Received</option>
              <option value="cancelled">Cancelled</option>
            </select>
          </div>
        </div>

        {/* Table */}
        <div className="purchases-table-wrap">
          <table className="purchases-table">
            <thead>
              <tr>
                <th>PO #</th>
                <th>PO Date</th>
                <th>Expected Del.</th>
                <th>Supplier ID</th>
                <th>Status</th>
                <th className="purchases-th--right">Total Amount</th>
                <th className="purchases-th--center">Actions</th>
              </tr>
            </thead>
            <tbody>
              {loading && (
                <tr>
                  <td colSpan={7} className="purchases-table__loading">
                    <div className="purchases-spinner" />
                    Loading purchase orders...
                  </td>
                </tr>
              )}
              
              {!loading && filteredPurchases.map((po) => (
                <tr key={po.id}>
                  <td>
                    <div style={{ fontWeight: 600, color: 'var(--c-slate-900)' }}>{po.po_number}</div>
                  </td>
                  <td className="purchases-td--muted">
                    {po.po_date ? new Date(po.po_date).toLocaleDateString() : '-'}
                  </td>
                  <td className="purchases-td--muted">
                    {po.expected_delivery_date ? new Date(po.expected_delivery_date).toLocaleDateString() : '-'}
                  </td>
                  <td className="purchases-td--muted" style={{ fontSize: '0.75rem' }}>
                    {po.supplier_id ? po.supplier_id.substring(0, 8) : '-'}...
                  </td>
                  <td>
                    <span className={`purchases-badge purchases-badge--${po.status}`}>{po.status}</span>
                  </td>
                  <td className="purchases-td--right">
                    ${(po.total_amount || 0).toFixed(2)}
                  </td>
                  <td className="purchases-td--center">
                    <div className="purchases-actions">
                      <button
                        className="purchases-action-btn purchases-action-btn--view"
                        onClick={() => navigate(`/purchases/${po.id}`)}
                        title="View PO"
                      >
                        <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2"><path d="M1 12s4-8 11-8 11 8 11 8-4 8-11 8-11-8-11-8z"/><circle cx="12" cy="12" r="3"/></svg>
                      </button>
                      <button
                        className="purchases-action-btn purchases-action-btn--del"
                        onClick={() => handleDelete(po.id)}
                        disabled={deletingId === po.id}
                        title="Delete PO"
                      >
                        <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2"><polyline points="3 6 5 6 21 6"/><path d="M19 6l-1 14H6L5 6"/><path d="M10 11v6"/><path d="M14 11v6"/><path d="M9 6V4h6v2"/></svg>
                      </button>
                    </div>
                  </td>
                </tr>
              ))}

              {!loading && filteredPurchases.length === 0 && (
                <tr>
                  <td colSpan={7} className="purchases-table__empty">
                    <svg width="40" height="40" viewBox="0 0 24 24" fill="none" stroke="var(--c-slate-300)" strokeWidth="1.5"><path d="M14 2H6a2 2 0 00-2 2v16a2 2 0 002 2h12a2 2 0 002-2V8l-6-6z"/><path d="M14 3v5h5M16 13H8M16 17H8M10 9H8"/></svg>
                    <p>No purchase orders found</p>
                    {search || statusFilter !== 'all' ? (
                      <button className="purchases-empty-link" onClick={() => { setSearch(''); setStatusFilter('all'); }}>Clear filters</button>
                    ) : (
                      <Link to="/purchases/new" className="purchases-empty-link">Create the first PO →</Link>
                    )}
                  </td>
                </tr>
              )}
            </tbody>
          </table>
        </div>
      </div>
      <div style={{ height: 32 }} />
    </div>
  );
}
