export interface Project {
  id: string;
  name: string;
  description: string;
  icon: string;
  color: string | null;
  asset_ids: string[];
  notes: ProjectNote[];
  deliverables: Deliverable[];
  created_at: string;
  updated_at: string;
}

export interface ProjectNote {
  id: string;
  title: string;
  content: string;
  pinned: boolean;
  created_at: string;
}

export interface Deliverable {
  id: string;
  name: string;
  description: string;
  path: string | null;
  asset_id: string | null;
  status: "Planned" | "InProgress" | "Review" | "Approved" | "Delivered" | "Cancelled";
  due_date: string | null;
}
