import { WorkspaceInfo } from '#/store';
import { defineStore } from 'pinia';
import { WORKSPACE_INFO_KEY } from '@/enums/cacheEnum';
import { Persistent } from '@/utils/cache/persistent';

interface WorkspaceState {
  workspaceInfo: Nullable<WorkspaceInfo>;
}

export const useWorkspaceStore = defineStore({
  id: 'app-workspace',
  state: (): WorkspaceState => ({
    workspaceInfo: Persistent.getLocal(WORKSPACE_INFO_KEY),
  }),
  getters: {
    getWorkspaceInfo(state): Nullable<WorkspaceInfo> {
      return state.workspaceInfo;
    },
  },
  actions: {
    setWorkspaceInfo(info: WorkspaceInfo) {
      this.workspaceInfo = Object.assign({}, this.workspaceInfo, info);
      Persistent.setLocal(WORKSPACE_INFO_KEY, this.workspaceInfo, true);
    },
    resetWorkspaceInfo() {
      Persistent.removeLocal(WORKSPACE_INFO_KEY, true);
      this.workspaceInfo = null;
    },
  },
});
