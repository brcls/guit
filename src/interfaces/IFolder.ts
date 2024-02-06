import { IFile } from "./IFile";

export interface IFolder {
  id: string;
  name: string;
  status: "staged" | "changed";
  files: Record<string, IFile>;
  folderId: string | null;
  folders: Record<string, IFolder> | null;
}
