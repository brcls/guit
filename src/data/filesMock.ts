import { IFolder } from "@/interfaces/IFolder";

// Mock do projeto React com IDs únicos preenchidos manualmente
export const filesMock: IFolder = {
  id: "1",
  name: "my-react-project",
  status: "changed",
  folderId: null,
  files: {
    "2": {
      name: "App",
      extension: "jsx",
      status: "changed",
      id: "2", // ID único
      folderId: "1", // Folder ID único
    },
    "3": {
      name: "index",
      extension: "js",
      status: "changed",
      id: "3", // ID único
      folderId: "1", // Folder ID único
    },
    "4": {
      name: "styles",
      extension: "css",
      status: "changed",
      id: "4", // ID único
      folderId: "1", // Folder ID único
    },
  },
  folders: {
    "5": {
      name: "src",
      folderId: "1",
      id: "5", // ID único
      status: "changed",
      folders: null,
      files: {
        "6": {
          name: "index",
          extension: "jsx",
          status: "changed",
          id: "6", // ID único
          folderId: "5", // Folder ID único
        },
        "7": {
          name: "components",
          extension: "jsx",
          status: "changed",
          id: "7", // ID único
          folderId: "5", // Folder ID único
        },
      },
    },
    "8": {
      name: "public",
      folderId: "1",
      id: "8", // ID único
      status: "changed",
      folders: {
        "15": {
          name: "testing",
          folderId: "8",
          id: "15", // ID único
          status: "changed",
          folders: {
            "27": {
              name: "ndf",
              folderId: "15",
              id: "27", // ID único
              status: "changed",
              folders: null,
              files: {
                "34": {
                  name: "index",
                  extension: "html",
                  status: "changed",
                  id: "34", // ID único
                  folderId: "27", // Folder ID único
                },
                "36": {
                  name: "favicon",
                  extension: "ico",
                  status: "changed",
                  id: "36", // ID único
                  folderId: "27", // Folder ID único
                },
              },
            },
          },
          files: {
            "19": {
              name: "index",
              extension: "html",
              status: "changed",
              id: "19", // ID único
              folderId: "15", // Folder ID único
            },
            "20": {
              name: "favicon",
              extension: "ico",
              status: "changed",
              id: "20", // ID único
              folderId: "15", // Folder ID único
            },
          },
        },
      },
      files: {
        "9": {
          name: "index",
          extension: "html",
          status: "changed",
          id: "9", // ID único
          folderId: "8", // Folder ID único
        },
        "10": {
          name: "favicon",
          extension: "ico",
          status: "changed",
          id: "10", // ID único
          folderId: "8", // Folder ID único
        },
      },
    },
  },
};
