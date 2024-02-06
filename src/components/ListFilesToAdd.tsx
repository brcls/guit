// ListFilesToAdd.jsx
import React, { useState } from "react";
import {
  Accordion,
  AccordionContent,
  AccordionItem,
  AccordionTrigger,
} from "./ui/accordion";
import { IFolder } from "@/interfaces/IFolder";
import { filesMock } from "@/data/filesMock";
import { Folder } from "./organismins/Folder";
import { IFile } from "@/interfaces/IFile";

export default function ListFilesToAdd() {
  const [changedRoot, setChangedRoot] = useState<IFolder | null>(filesMock);
  const [stagedRoot, setStagedRoot] = useState<IFolder | null>(null);

  const searchFolderRootToNode = (
    root: IFolder | null,
    id: string | null,
  ): IFolder | null => {
    if (root === null || id === null) {
      return null;
    }

    if (root.id === id) {
      return root;
    }

    if (root.folders) {
      const childFolder = root.folders[id];

      if (childFolder) return childFolder;

      for (const folder of Object.values(root.folders)) {
        const result = searchFolderRootToNode(folder, id);
        if (result) return result;
      }
    }

    return null;
  };

  const searchFolderNodeToRoot = (
    node: IFolder | null,
    id: string | null,
  ): IFolder | null => {
    if (node === null || id === null) {
      return null;
    }

    if (node.id === id) {
      return node;
    }

    if (node.folderId) {
      const parent = searchFolderRootToNode(changedRoot, node.folderId);
      return searchFolderNodeToRoot(parent, id);
    }

    return null;
  };

  const deleteFile = (fileToAdd: IFile, currentFolder: IFolder) => {
    delete currentFolder.files[fileToAdd.id];

    const findParent = searchFolderRootToNode(
      changedRoot,
      currentFolder.folderId,
    );

    const currentFoldersIsNull = !currentFolder.folders;
    const currentFilesIsEmpty = !Object.keys(currentFolder.files)?.length;
    const currentFoldersIsEmpty =
      currentFolder.folders && !Object.keys(currentFolder?.folders)?.length;

    if (
      findParent?.folders &&
      currentFilesIsEmpty &&
      (currentFoldersIsNull || currentFoldersIsEmpty)
    )
      delete findParent.folders[currentFolder.id];
  };

  const addOnRootEmpty = (fileToAdd: IFile, currentFolder: IFolder) => {
    if (stagedRoot === null) {
      const updatedRoot: IFolder = {
        ...currentFolder,
        files: { [fileToAdd.id]: fileToAdd },
        folders: null,
      };

      setStagedRoot(updatedRoot);
      deleteFile(fileToAdd, currentFolder);
      return true;
    }
    return false;
  };

  const addOnCurrentRoot = (fileToAdd: IFile, currentFolder: IFolder) => {
    if (stagedRoot?.id === currentFolder?.id) {
      // A pasta Root é a atual pasta
      const isFileAlredyInFolder = !!stagedRoot.files[fileToAdd.id];

      // O arquivo já esta na pasta
      if (isFileAlredyInFolder) return false;

      setStagedRoot((prev) => {
        if (prev) {
          const updatedRoot = { ...prev };
          updatedRoot.files[fileToAdd.id] = fileToAdd;
          return updatedRoot;
        }
        return null;
      });

      deleteFile(fileToAdd, currentFolder);
      return true;
    }

    return false;
  };

  const addChildrenOnParentRoot = (
    fileToAdd: IFile,
    currentFolder: IFolder,
  ) => {
    const searchToRoot = searchFolderNodeToRoot(stagedRoot, currentFolder.id);

    if (searchToRoot && stagedRoot) {
      const updatedRoot: IFolder = {
        ...searchToRoot,
        files: { [fileToAdd.id]: fileToAdd },
        folders: { [stagedRoot.id]: stagedRoot },
      };

      setStagedRoot(updatedRoot);
      deleteFile(fileToAdd, currentFolder);
      return true;
    }
    return false;
  };

  const addParentInChildrenRoot = (
    fileToAdd: IFile,
    currentFolder: IFolder,
  ) => {
    const searchToNodeInStaged = searchFolderRootToNode(
      stagedRoot,
      currentFolder.id,
    );

    if (searchToNodeInStaged && stagedRoot) {
      if (stagedRoot.folders)
        setStagedRoot((prev) => {
          if (prev) {
            const updatedRoot = { ...prev };

            console.log(updatedRoot, searchToNodeInStaged);

            if (
              updatedRoot.folders &&
              updatedRoot.folders[searchToNodeInStaged.id]
            )
              updatedRoot.folders[searchToNodeInStaged.id].files[fileToAdd.id] =
                fileToAdd;

            return updatedRoot;
          }
          return null;
        });

      deleteFile(fileToAdd, currentFolder);
      return true;
    }
    return false;
  };

  const addParentInChildrenWithoutFolder = (
    fileToAdd: IFile,
    currentFolder: IFolder,
  ) => {
    const searchToNodeInChanged = searchFolderRootToNode(
      changedRoot,
      currentFolder.id,
    );

    if (searchToNodeInChanged && stagedRoot) {
      if (!stagedRoot.folders) {
        setStagedRoot((prev) => {
          if (prev) {
            const updatedRoot = { ...prev };

            searchToNodeInChanged.files[fileToAdd.id] = fileToAdd;
            updatedRoot.folders = {
              [searchToNodeInChanged.id]: searchToNodeInChanged,
            };

            return updatedRoot;
          }
          return null;
        });

        deleteFile(fileToAdd, currentFolder);
        return true;
      }
    }
    return false;
  };

  const addParentInChildrenWithFolder = (
    fileToAdd: IFile,
    currentFolder: IFolder,
  ) => {
    const searchToNodeInChanged = searchFolderRootToNode(
      changedRoot,
      currentFolder.id,
    );

    if (searchToNodeInChanged && stagedRoot) {
      if (stagedRoot.folders) {
        setStagedRoot((prev) => {
          if (prev) {
            const updatedRoot = { ...prev };

            if (
              updatedRoot.folders &&
              updatedRoot.folders[searchToNodeInChanged.id]
            ) {
              updatedRoot.folders[searchToNodeInChanged.id].files[
                fileToAdd.id
              ] = fileToAdd;
            }

            return updatedRoot;
          }
          return null;
        });

        deleteFile(fileToAdd, currentFolder);
        return true;
      }
    }
    return false;
  };
  const handleAddFile = (fileToAdd: IFile, currentFolder: IFolder) => {
    console.log(1);
    if (addOnRootEmpty(fileToAdd, currentFolder)) return;
    console.log(2);
    if (addOnCurrentRoot(fileToAdd, currentFolder)) return;
    console.log(3);
    if (addChildrenOnParentRoot(fileToAdd, currentFolder)) return;
    console.log(4);
    if (addParentInChildrenRoot(fileToAdd, currentFolder)) return;
    console.log(5);
    if (addParentInChildrenWithoutFolder(fileToAdd, currentFolder)) return;
    console.log(6);
    if (addParentInChildrenWithFolder(fileToAdd, currentFolder)) return;
    console.log(7);
  };

  return (
    <div className="flex w-full gap-4 p-4">
      <Accordion type="multiple" className="w-1/3">
        <AccordionItem value="item-1">
          <AccordionTrigger>Staged</AccordionTrigger>
          <AccordionContent>
            <Folder folder={stagedRoot} handleAddFile={handleAddFile} />
          </AccordionContent>
        </AccordionItem>
        <AccordionItem value="item-2">
          <AccordionTrigger>Changes</AccordionTrigger>
          <AccordionContent>
            <Folder folder={changedRoot} handleAddFile={handleAddFile} />
          </AccordionContent>
        </AccordionItem>
      </Accordion>
    </div>
  );
}
