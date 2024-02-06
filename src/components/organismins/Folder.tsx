"use client";

import * as React from "react";
import { ChevronsUpDown, FolderIcon, Minus, Plus } from "lucide-react";
import {
  Collapsible,
  CollapsibleContent,
  CollapsibleTrigger,
} from "@/components/ui/collapsible";
import File from "../molecules/File";
import { IFolder } from "@/interfaces/IFolder";
import { IFile } from "@/interfaces/IFile";

interface IFolderComponentProps {
  folder: IFolder | null | undefined;
  handleAddFile: (file: IFile, folder: IFolder) => void;
}

export function Folder({ folder, handleAddFile }: IFolderComponentProps) {
  const [isOpen, setIsOpen] = React.useState(true);

  return (
    <Collapsible
      open={isOpen}
      onOpenChange={setIsOpen}
      className="w-full space-y-2"
    >
      {folder && folder !== undefined && (
        <>
          <FolderHeader folder={folder} />
          <CollapsibleContent className="mr-6 space-y-2">
            {folder?.files &&
              Object.values(folder?.files).map((file) => (
                <File
                  key={file.id}
                  folder={folder}
                  file={file}
                  handleAddFile={handleAddFile}
                />
              ))}
            {folder?.folders &&
              Object?.values(folder?.folders)?.map((folder) => (
                <Folder
                  key={folder.id}
                  folder={folder}
                  handleAddFile={handleAddFile}
                />
              ))}
          </CollapsibleContent>
        </>
      )}
    </Collapsible>
  );
}

interface IFolderHeaderProps {
  folder: IFolder;
}

function FolderHeader({ folder }: IFolderHeaderProps) {
  return (
    <div className="outline-dark flex items-center justify-between">
      <div className="flex items-center justify-start gap-2 text-xs">
        <FolderIcon size={15} /> {folder.name}
      </div>

      <div className="flex gap-2">
        <button className="outline-dark-hover flex items-center justify-center rounded-md p-1">
          {folder.status === "changed" ? (
            <Plus width={15} height={15} />
          ) : (
            <Minus width={15} height={15} />
          )}
        </button>
        <CollapsibleTrigger asChild>
          <button className="outline-dark-hover flex items-center justify-center rounded-md p-1">
            <ChevronsUpDown size={15} />
            <span className="sr-only">Toggle</span>
          </button>
        </CollapsibleTrigger>
      </div>
    </div>
  );
}
