import type { WorkspaceInfo } from '#/types';

import { defineStore } from 'pinia';

interface AccessState {
  id?: string;
  workspaces: WorkspaceInfo[];
}

/**
 * workspace store
 */
export const useWorkspaceStore = defineStore('workspace', {
  actions: {
    getId() {
      return this.id;
    },
    setId(id: string) {
      this.id = id;
    },
    setWorkspaces(workspaceInfo: WorkspaceInfo[]) {
      this.workspaces = workspaceInfo;
      if (this.id) {
        const result = this.workspaces.find(
          (workspace) => workspace.id === this.id,
        );
        if (!result) {
          this.id = this.workspaces[0]?.id;
        }
      } else {
        this.id = this.workspaces[0]?.id;
      }
    },
    getWorkspace() {
      return this.workspaces.find((workspace) => workspace.id === this.id);
    },
  },
  state: (): AccessState => ({
    workspaces: [],
  }),
});
