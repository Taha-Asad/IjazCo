import React, { useState, useRef } from 'react';
import { useNavigate, Link } from 'react-router-dom';
import { uploadImport } from '../services/imports';
import { useToast } from '../contexts/ToastContext';
import './ImportsPage.css'; // Reusing styles

export default function NewImportPage() {
  const [entityType, setEntityType] = useState('inventory');
  const [file, setFile] = useState<File | null>(null);
  const [isUploading, setIsUploading] = useState(false);
  const [isDragActive, setIsDragActive] = useState(false);
  const fileInputRef = useRef<HTMLInputElement>(null);
  const navigate = useNavigate();
  const { success, error } = useToast();

  const handleDragEnter = (e: React.DragEvent) => {
    e.preventDefault();
    e.stopPropagation();
    setIsDragActive(true);
  };

  const handleDragLeave = (e: React.DragEvent) => {
    e.preventDefault();
    e.stopPropagation();
    setIsDragActive(false);
  };

  const handleDrop = (e: React.DragEvent) => {
    e.preventDefault();
    e.stopPropagation();
    setIsDragActive(false);
    
    if (e.dataTransfer.files && e.dataTransfer.files.length > 0) {
      const droppedFile = e.dataTransfer.files[0];
      validateAndSetFile(droppedFile);
    }
  };

  const handleFileChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    if (e.target.files && e.target.files.length > 0) {
      validateAndSetFile(e.target.files[0]);
    }
  };

  const validateAndSetFile = (selectedFile: File) => {
    if (!selectedFile.name.endsWith('.csv') && !selectedFile.name.endsWith('.xlsx')) {
      error('Please upload a CSV or Excel file.');
      return;
    }
    setFile(selectedFile);
  };

  const handleUpload = async () => {
    if (!file) return;
    
    try {
      setIsUploading(true);
      await uploadImport(entityType, file);
      success('File uploaded successfully. Processing will begin shortly.');
      navigate('/imports');
    } catch (err: any) {
      error(err.message || 'Failed to upload file');
    } finally {
      setIsUploading(false);
    }
  };

  return (
    <div className="imports-page animate-fade-in">
      <div className="imports-header">
        <div>
          <h1 className="imports-header__title">Upload Data</h1>
          <p className="imports-header__subtitle">Bulk import your data using a CSV file.</p>
        </div>
      </div>

      <div className="upload-container">
        <div className="upload-card">
          <div className="form-group">
            <label htmlFor="entityType">What are you importing?</label>
            <select
              id="entityType"
              className="form-select"
              value={entityType}
              onChange={(e) => setEntityType(e.target.value)}
            >
              <option value="inventory">Inventory Items</option>
              <option value="customers">Customers</option>
              <option value="suppliers">Suppliers</option>
              <option value="categories">Categories</option>
            </select>
          </div>

          <div 
            className={`dropzone ${isDragActive ? 'dropzone--active' : ''}`}
            onDragEnter={handleDragEnter}
            onDragOver={(e) => e.preventDefault()}
            onDragLeave={handleDragLeave}
            onDrop={handleDrop}
            onClick={() => fileInputRef.current?.click()}
          >
            <svg width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.5">
              <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"></path>
              <polyline points="17 8 12 3 7 8"></polyline>
              <line x1="12" y1="3" x2="12" y2="15"></line>
            </svg>
            <h4>Click to upload or drag and drop</h4>
            <p>CSV or Excel files only (max. 10MB)</p>
            <input 
              type="file" 
              ref={fileInputRef} 
              onChange={handleFileChange}
              accept=".csv, application/vnd.openxmlformats-officedocument.spreadsheetml.sheet, application/vnd.ms-excel"
            />
          </div>

          {file && (
            <div className="selected-file animate-slide-up">
              <div className="selected-file-info">
                <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" style={{color: 'var(--c-brand)'}}>
                  <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"></path>
                  <polyline points="14 2 14 8 20 8"></polyline>
                </svg>
                <div>
                  <div className="selected-file-name">{file.name}</div>
                  <div className="selected-file-size">{(file.size / 1024).toFixed(1)} KB</div>
                </div>
              </div>
              <button className="btn-clear" onClick={() => setFile(null)}>
                <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2"><line x1="18" y1="6" x2="6" y2="18"></line><line x1="6" y1="6" x2="18" y2="18"></line></svg>
              </button>
            </div>
          )}

          <div className="upload-actions">
            <Link to="/imports" className="btn-secondary">Cancel</Link>
            <button 
              className="btn-primary" 
              disabled={!file || isUploading}
              onClick={handleUpload}
            >
              {isUploading ? 'Uploading...' : 'Start Import'}
            </button>
          </div>
        </div>
      </div>
    </div>
  );
}
