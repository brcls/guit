export interface IFile {
  id: string;
  name: string;
  extension: string;
  status: "staged" | "changed";
  folderId: string;
}
