// FileItem.jsx
import { Minus, Plus } from "lucide-react";
import { IFile } from "@/interfaces/IFile";
import { IFolder } from "@/interfaces/IFolder";

interface IFileComponentProps {
  file: IFile;
  folder: IFolder;
  handleAddFile: (file: IFile, folder: IFolder) => void;
}

const File = ({ file, folder, handleAddFile }: IFileComponentProps) => {
  return (
    <div className="outline-dark flex w-full items-center justify-between gap-4 pl-6">
      <div className="flex w-min items-center text-nowrap">{file.name}</div>
      <button
        onClick={() => handleAddFile(file, folder)}
        className="outline-dark-hover flex items-center justify-center rounded-md p-1"
      >
        {file.status === "changed" ? (
          <Plus width={15} height={15} />
        ) : (
          <Minus width={15} height={15} />
        )}
      </button>
    </div>
  );
};

export default File;
