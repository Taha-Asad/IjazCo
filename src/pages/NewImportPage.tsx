import React, { useState } from 'react';
import { useNavigate, Link } from 'react-router-dom';
import { createImport } from '../services/imports';
import { getSuppliers, type Supplier } from '../services/suppliers';
import { useToast } from '../contexts/ToastContext';
import './ImportsPage.css'; // Reusing styles

export default function NewImportPage() {
  const [isSubmitting, setIsSubmitting] = useState(false);
  const [suppliers, setSuppliers] = useState<Supplier[]>([]);
  const [form, setForm] = useState({
    supplier_id: '',
    po_id: '',
    shipment_date: new Date().toISOString().slice(0, 10),
    arrival_date: '',
    shipping_method: 'Air',
    tracking_number: '',
    container_number: '',
    freight_cost: '',
    insurance_cost: '',
    customs_duty: '',
    other_charges: '',
    notes: '',
  });
  const navigate = useNavigate();
  const { success, error } = useToast();

  React.useEffect(() => {
    getSuppliers({ limit: 100 }).then((res) => setSuppliers(res.suppliers || [])).catch(() => {
      setSuppliers([]);
    });
  }, []);

  const handleChange = (e: React.ChangeEvent<HTMLInputElement | HTMLTextAreaElement | HTMLSelectElement>) => {
    setForm((prev) => ({ ...prev, [e.target.name]: e.target.value }));
  };

  const handleSubmit = async () => {
    try {
      if (!form.supplier_id) {
        error('Supplier is required.');
        return;
      }
      setIsSubmitting(true);
      await createImport({
        supplier_id: form.supplier_id,
        po_id: form.po_id || undefined,
        shipment_date: form.shipment_date || undefined,
        arrival_date: form.arrival_date || undefined,
        shipping_method: form.shipping_method || undefined,
        tracking_number: form.tracking_number || undefined,
        container_number: form.container_number || undefined,
        freight_cost: form.freight_cost ? Number(form.freight_cost) : undefined,
        insurance_cost: form.insurance_cost ? Number(form.insurance_cost) : undefined,
        customs_duty: form.customs_duty ? Number(form.customs_duty) : undefined,
        other_charges: form.other_charges ? Number(form.other_charges) : undefined,
        notes: form.notes || undefined,
      });
      success('Import order created successfully.');
      navigate('/imports');
    } catch (err: any) {
      error(err.message || 'Failed to create import order');
    } finally {
      setIsSubmitting(false);
    }
  };

  return (
    <div className="imports-page animate-fade-in">
      <div className="imports-header">
        <div>
          <h1 className="imports-header__title">Create Import Order</h1>
          <p className="imports-header__subtitle">Capture shipping, duty and ETA details for inbound stock.</p>
        </div>
      </div>

      <div className="upload-container">
        <div className="upload-card">
          <div className="form-grid">
            <div className="form-group">
              <label htmlFor="supplier_id">Supplier *</label>
              <select id="supplier_id" name="supplier_id" className="form-select" value={form.supplier_id} onChange={handleChange} required>
                <option value="">Select supplier</option>
                {suppliers.map((s) => (
                  <option key={s.id} value={s.id}>{s.name}</option>
                ))}
              </select>
            </div>
            <div className="form-group">
              <label htmlFor="po_id">Purchase Order ID</label>
              <input id="po_id" name="po_id" className="form-input" value={form.po_id} onChange={handleChange} placeholder="Optional PO UUID" />
            </div>
            <div className="form-group">
              <label htmlFor="shipment_date">Shipment Date</label>
              <input id="shipment_date" name="shipment_date" type="date" className="form-input" value={form.shipment_date} onChange={handleChange} />
            </div>
            <div className="form-group">
              <label htmlFor="arrival_date">Arrival Date</label>
              <input id="arrival_date" name="arrival_date" type="date" className="form-input" value={form.arrival_date} onChange={handleChange} />
            </div>
            <div className="form-group">
              <label htmlFor="shipping_method">Shipping Method</label>
              <select id="shipping_method" name="shipping_method" className="form-select" value={form.shipping_method} onChange={handleChange}>
                <option value="Air">Air</option>
                <option value="Sea">Sea</option>
                <option value="Land">Land</option>
              </select>
            </div>
            <div className="form-group">
              <label htmlFor="tracking_number">Tracking Number</label>
              <input id="tracking_number" name="tracking_number" className="form-input" value={form.tracking_number} onChange={handleChange} />
            </div>
            <div className="form-group">
              <label htmlFor="container_number">Container Number</label>
              <input id="container_number" name="container_number" className="form-input" value={form.container_number} onChange={handleChange} />
            </div>
            <div className="form-group">
              <label htmlFor="freight_cost">Freight Cost</label>
              <input id="freight_cost" name="freight_cost" type="number" min="0" step="0.01" className="form-input" value={form.freight_cost} onChange={handleChange} />
            </div>
            <div className="form-group">
              <label htmlFor="insurance_cost">Insurance Cost</label>
              <input id="insurance_cost" name="insurance_cost" type="number" min="0" step="0.01" className="form-input" value={form.insurance_cost} onChange={handleChange} />
            </div>
            <div className="form-group">
              <label htmlFor="customs_duty">Customs Duty</label>
              <input id="customs_duty" name="customs_duty" type="number" min="0" step="0.01" className="form-input" value={form.customs_duty} onChange={handleChange} />
            </div>
            <div className="form-group">
              <label htmlFor="other_charges">Other Charges</label>
              <input id="other_charges" name="other_charges" type="number" min="0" step="0.01" className="form-input" value={form.other_charges} onChange={handleChange} />
            </div>
            <div className="form-group form-group--full">
              <label htmlFor="notes">Notes</label>
              <textarea id="notes" name="notes" className="form-input" value={form.notes} onChange={handleChange} rows={3} />
            </div>
          </div>

          <div className="upload-actions">
            <Link to="/imports" className="btn-secondary">Cancel</Link>
            <button 
              className="btn-primary" 
              disabled={isSubmitting}
              onClick={handleSubmit}
            >
              {isSubmitting ? 'Creating...' : 'Create Import'}
            </button>
          </div>
        </div>
      </div>
    </div>
  );
}
