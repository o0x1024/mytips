<template>
  <div class="h-full flex flex-col border-r border-base-300 relative">
    <!-- 顶部区域：笔记统计和快速操作 (仅桌面端显示) -->
    <div v-if="!isMobile" class="p-3 border-b border-base-300 bg-base-100">
      <!-- 笔记统计信息 -->
      <div class="flex items-center justify-between mb-5">
        <div class="flex items-center gap-2 mt-2">
          <h2 class="text-lg font-semibold text-base-content">{{ t('noteList.notes') }}</h2>
          <div class="badge badge-outline">{{ props.totalCount > 0 ? props.totalCount : filteredNotes.length }}</div>
        </div>
        <div class="flex items-center gap-1">
          <!-- 快速新建按钮 -->
          <button 
            class="btn btn-sm btn-ghost" 
            @click="$emit('new-note')" 
            :title="t('noteList.newNoteTooltip')">
            <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
            </svg>
          </button>
          <!-- 刷新按钮 -->
          <button 
            class="btn btn-sm btn-ghost" 
            @click="$emit('refresh')" 
            :title="t('noteList.refreshList')"
            >
            <svg v-if="!loading" xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
            </svg>
          </button>
        </div>
      </div>
      
      <!-- 快速搜索框 -->
      
    </div>

    <!-- 工具栏：排序和视图切换 -->
    <div class="p-1 bg-base-200 border-base-300 border-b flex items-center justify-between">
      <!-- 排序控制 -->
      <div class="dropdown">
        <label tabindex="0" class="btn btn-sm btn-ghost m-1">
          <span class="mr-1">{{ t('noteList.sort') }}</span>
          <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7" />
          </svg>
        </label>
        <ul tabindex="0" class="dropdown-content z-10 menu p-2 shadow bg-base-100 rounded-box w-52">
          <li><a @click="sortBy('updated', 'desc')" :class="{'font-bold': sortField === 'updated' && sortOrder === 'desc'}">{{ t('noteList.sortBy.updated') }}</a></li>
          <li><a @click="sortBy('created', 'desc')" :class="{'font-bold': sortField === 'created' && sortOrder === 'desc'}">{{ t('noteList.sortBy.created') }}</a></li>
          <li><a @click="sortBy('title', 'asc')" :class="{'font-bold': sortField === 'title' && sortOrder === 'asc'}">{{ t('noteList.sortBy.titleAsc') }}</a></li>
          <li><a @click="sortBy('title', 'desc')" :class="{'font-bold': sortField === 'title' && sortOrder === 'desc'}">{{ t('noteList.sortBy.titleDesc') }}</a></li>
        </ul>
      </div>
    </div>

    <!-- 笔记列表 -->
    <div ref="noteListContainer" class="flex-1 overflow-y-auto" @scroll="handleScroll">
      <!-- 加载状态 -->
      <div v-if="loading && filteredNotes.length === 0" class="flex justify-center items-center h-32">
        <span class="loading loading-spinner loading-md"></span>
      </div>

      <!-- 无结果提示 -->
      <div v-else-if="filteredNotes.length === 0" class="flex flex-col items-center justify-center py-10 text-base-content/80">
        <svg xmlns="http://www.w3.org/2000/svg" class="h-16 w-16 mb-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1" d="M9 13h6m-3-3v6m-9 1V7a2 2 0 012-2h6l2 2h6a2 2 0 012 2v8a2 2 0 01-2 2H5a2 2 0 01-2-2z" />
        </svg>
        <p class="text-lg mb-2">{{ t('noteList.noNotesFound') }}</p>
        <!-- <button class="btn btn-sm btn-primary" @click="$emit('new-note')">创建新笔记</button> -->
      </div>
      
      <div v-if="virtualizer" :style="{ height: `${totalSize}px`, width: '100%', position: 'relative' }">
        <div 
          v-for="virtualRow in virtualItems" 
          :key="filteredNotes[virtualRow.index]?.id || `index-${virtualRow.index}`"
          :data-index="virtualRow.index"
          :ref="(el) => el && virtualizer.measureElement(el as Element)"
          :style="{
            position: 'absolute',
            top: 0,
            left: 0,
            width: '100%',
            height: `${virtualRow.size}px`,
            transform: `translateY(${virtualRow.start}px)`
          }"
          class="p-2 cursor-pointer hover:bg-base-200 transition-colors border-b border-dashed border-base-300 min-h-[60px] flex flex-col note-list-item"
          :class="{'bg-primary/10': selectedNoteId === filteredNotes[virtualRow.index]?.id}"
          @click="filteredNotes[virtualRow.index] && selectNote(filteredNotes[virtualRow.index])"
          @contextmenu="filteredNotes[virtualRow.index] && openContextMenu($event, filteredNotes[virtualRow.index])">
          
          <div v-if="filteredNotes[virtualRow.index]" class="flex items-center justify-between">
            <h3 class="font-medium">{{ filteredNotes[virtualRow.index].title || t('noteList.untitled') }}</h3>
            <div class="flex items-center gap-2">
              <span class="text-xs text-base-content/80">{{ formatDate(filteredNotes[virtualRow.index].updated_at) }}</span>
              <!-- 加密状态指示器 -->
              <span v-if="isNoteEncrypted(filteredNotes[virtualRow.index].id)" :title="t('noteList.encrypted')" class="text-warning">
                <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z" />
                </svg>
              </span>
              <span v-if="filteredNotes[virtualRow.index].isPinned" :title="t('noteList.pinned')" class="text-warning">
                <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 5a2 2 0 012-2h10a2 2 0 012 2v16l-7-3.5L5 21V5z" />
                </svg>
              </span>
            </div>
          </div>
          <p v-if="filteredNotes[virtualRow.index]" class="line-clamp-1 text-xs text-base-content/80 mt-1">{{ getPreviewContent(filteredNotes[virtualRow.index]) }}</p>
        </div>
      </div>

      <!-- 列表底部的加载更多提示 -->
      <div v-if="loading && filteredNotes.length > 0" class="flex justify-center items-center p-4">
        <span class="loading loading-spinner loading-sm"></span>
        <span class="ml-2 text-sm text-base-content/70">{{ t('noteList.loading') }}</span>
      </div>
      <div v-if="!hasMoreTips && filteredNotes.length > 0" class="text-center p-4 text-sm text-base-content/50">
        {{ t('noteList.noMoreNotes') }}
      </div>

    </div>

    <!-- 底部动作区 -->
    <div class="p-2  border-t border-base-200 ">
      <button class="btn btn-primary btn-block btn-md" @click="$emit('new-note')">
        <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 mr-1" fill="none" viewBox="0 0 24 24" stroke="currentColor">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
        </svg>
        {{ t('noteList.createNewNote') }}
      </button>
    </div>

    <!-- 右键菜单 -->
    <Teleport to="body">
    <div v-if="showContextMenu" 
        class="context-menu fixed bg-base-100 shadow-lg rounded-lg p-2 border border-base-300"
        :style="{ top: contextMenuY + 'px', left: contextMenuX + 'px', zIndex: 9999 }">
      <!-- 当前操作笔记信息头部 -->
      <!-- <div v-if="contextNote" class="px-3 py-2 bg-base-200 rounded-lg mb-2 border-b border-base-300">
        <div class="flex items-center gap-2 text-sm">
          <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4 text-primary" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
          </svg>
          <div class="flex-1 min-w-0">
            <div class="font-medium text-base-content truncate">
              {{ contextNote.title || '无标题' }}
            </div>
            <div class="text-xs text-base-content/60">
              {{ formatDate(contextNote.updated_at) }}
            </div>
          </div>
          <div v-if="contextNote.id === selectedNoteId" class="badge badge-primary badge-xs">当前选中</div> 
        </div>
      </div> -->
      
      <ul class="menu w-56 p-0">
        <!-- 加密/解密选项 -->
        <li v-if="contextNote">
          <a v-if="!isNoteEncrypted(contextNote.id)" @click="encryptContextNote" class="flex items-center gap-1 text-warning hover:bg-warning hover:text-warning-content">
            <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z" />
            </svg>
            {{ t('noteList.encryptNote') }}
          </a>
          <a v-else @click="decryptContextNote" class="flex items-center gap-1 text-info hover:bg-info hover:text-info-content">
            <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 11V7a4 4 0 118 0m-4 8v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2z" />
            </svg>
            {{ t('noteList.decryptNote') }}
          </a>
        </li>
        
        <li class="menu-title pt-2 pb-1 text-xs">{{ t('noteList.noteActions') }}</li>
        <li><a @click="deleteContextNote" class="flex items-center gap-1 text-error hover:bg-error hover:text-error-content">
          <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
          </svg>
          {{ t('noteList.deleteNote') }}
        </a></li>
        <li class="menu-title pt-2 pb-1 text-xs">{{ t('noteList.moveToCategory') }}</li>
        <li class="dropdown dropdown-hover">
          <div tabindex="0" role="button" class="flex items-center gap-1 justify-between p-2 hover:bg-base-200 rounded-lg">
            <div class="flex items-center gap-1">
              <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-2-2H5a2 2 0 00-2 2z" />
              </svg>
              {{ t('noteList.selectCategory') }}
            </div>
            <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" />
            </svg>
          </div>
          <ul tabindex="0" class="dropdown-content menu z-[2] shadow-lg bg-base-100 rounded-box w-52 p-2 absolute left-[calc(100%-12px)] top-0 -mt-2">
            <template v-for="notebook in notebooks" :key="notebook.id">
              <!-- 如果没有子分类，直接显示为普通菜单项 -->
              <li v-if="!notebook.children || notebook.children.length === 0">
                <a @click="moveNoteToCategory(notebook.id)" class="whitespace-nowrap flex items-center gap-2">
                  <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-2-2H5a2 2 0 00-2 2z" />
                  </svg>
                  {{ notebook.name }}
                </a>
              </li>
              
              <!-- 如果有子分类，显示为下拉菜单 -->
              <li v-else class="dropdown dropdown-hover relative"
                  @mouseenter="handleDropdownEnter(notebook.id)"
                  @mouseleave="handleDropdownLeave(notebook.id)"
                  :class="{ 'dropdown-open': openDropdowns.includes(notebook.id) }">
                <div tabindex="0" role="button" class="dropdown-trigger flex items-center gap-1 justify-between p-2 hover:bg-base-200 rounded-lg">
                  <div class="flex items-center gap-2">
                    <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-2-2H5a2 2 0 00-2 2z" />
                    </svg>
                  <span class="whitespace-nowrap">{{ notebook.name }}</span>
                  </div>
                  <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" />
                  </svg>
                </div>
                
                <!-- 子菜单 -->
                <ul tabindex="0" class="dropdown-content menu z-[3] shadow-lg bg-base-100 rounded-box w-52 p-2 absolute left-[calc(100%-12px)] top-0 -mt-2">
                  <!-- 父分类本身也可以选择 -->
                  <li class="menu-title text-xs pb-1">{{ t('noteList.selectParentCategory', { name: notebook.name }) }}</li>
                  <li>
                    <a @click="moveNoteToCategory(notebook.id)" class="whitespace-nowrap flex items-center gap-2 font-medium text-primary">
                      <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-2-2H5a2 2 0 00-2 2z" />
                      </svg>
                      {{ notebook.name }}
                    </a>
                  </li>
                  
                  <li class="menu-title text-xs pt-2 pb-1">{{ t('noteList.subcategories') }}</li>
                  <!-- 子分类列表 -->
                  <template v-for="child in notebook.children" :key="child.id">
                    <!-- 如果子分类没有更深层的子分类 -->
                    <li v-if="!child.children || child.children.length === 0">
                      <a @click="moveNoteToCategory(child.id)" class="whitespace-nowrap flex items-center gap-2">
                        <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-2-2H5a2 2 0 00-2 2z" />
                        </svg>
                        {{ child.name }}
                      </a>
                    </li>
                    
                    <!-- 如果子分类还有更深层的子分类 -->
                    <li v-else class="dropdown dropdown-hover relative"
                        @mouseenter="handleDropdownEnter(child.id)"
                        @mouseleave="handleDropdownLeave(child.id)"
                        :class="{ 'dropdown-open': openDropdowns.includes(child.id) }">
                      <div tabindex="0" role="button" class="dropdown-trigger flex items-center gap-1 justify-between p-2 hover:bg-base-200 rounded-lg">
                        <div class="flex items-center gap-2">
                          <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-2-2H5a2 2 0 00-2 2z" />
                          </svg>
                        <span class="whitespace-nowrap">{{ child.name }}</span>
                        </div>
                        <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" />
                        </svg>
                      </div>
                      
                      <!-- 三级菜单 -->
                      <ul tabindex="0" class="dropdown-content menu z-[4] shadow-lg bg-base-100 rounded-box w-52 p-2 absolute left-[calc(100%-12px)] top-0 -mt-2">
                        <!-- 当前分类本身 -->
                        <li class="menu-title text-xs pb-1">{{ t('noteList.selectParentCategory', { name: child.name }) }}</li>
                        <li>
                          <a @click="moveNoteToCategory(child.id)" class="whitespace-nowrap flex items-center gap-2 font-medium text-primary">
                            <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-2-2H5a2 2 0 00-2 2z" />
                            </svg>
                            {{ child.name }}
                          </a>
                        </li>
                        
                        <li class="menu-title text-xs pt-2 pb-1">{{ t('noteList.subcategories') }}</li>
                        <!-- 孙分类 -->
                        <template v-for="grandchild in child.children" :key="grandchild.id">
                          <li v-if="!grandchild.children || grandchild.children.length === 0">
                            <a @click="moveNoteToCategory(grandchild.id)" class="whitespace-nowrap flex items-center gap-2">
                              <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-2-2H5a2 2 0 00-2 2z" />
                              </svg>
                              {{ grandchild.name }}
                            </a>
                          </li>
                          
                          <!-- 如果还有更深层级 -->
                          <li v-else class="dropdown dropdown-hover relative"
                              @mouseenter="handleDropdownEnter(grandchild.id)"
                              @mouseleave="handleDropdownLeave(grandchild.id)"
                              :class="{ 'dropdown-open': openDropdowns.includes(grandchild.id) }">
                            <div tabindex="0" role="button" class="dropdown-trigger flex items-center gap-1 justify-between p-2 hover:bg-base-200 rounded-lg">
                              <div class="flex items-center gap-2">
                                <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-2-2H5a2 2 0 00-2 2z" />
                                </svg>
                              <span class="whitespace-nowrap">{{ grandchild.name }}</span>
                              </div>
                              <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" />
                              </svg>
                            </div>
                            
                            <!-- 四级菜单 -->
                            <ul tabindex="0" class="dropdown-content menu z-[5] shadow-lg bg-base-100 rounded-box w-52 p-2 absolute left-[calc(100%-12px)] top-0 -mt-2">
                              <!-- 当前分类本身 -->
                              <li class="menu-title text-xs pb-1">{{ t('noteList.selectParentCategory', { name: grandchild.name }) }}</li>
                              <li>
                                <a @click="moveNoteToCategory(grandchild.id)" class="whitespace-nowrap flex items-center gap-2 font-medium text-primary">
                                  <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-2-2H5a2 2 0 00-2 2z" />
                                  </svg>
                                  {{ grandchild.name }}
                                </a>
                              </li>
                              
                              <li class="menu-title text-xs pt-2 pb-1">{{ t('noteList.subcategories') }}</li>
                              <!-- 最深层分类 -->
                              <li v-for="item in grandchild.children" :key="item.id">
                                <a @click="moveNoteToCategory(item.id)" class="whitespace-nowrap flex items-center gap-2">
                                  <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-2-2H5a2 2 0 00-2 2z" />
                                  </svg>
                                  {{ item.name }}
                                </a>
                              </li>
                            </ul>
                          </li>
                        </template>
                      </ul>
                    </li>
                  </template>
                </ul>
              </li>
            </template>
          </ul>
        </li>
        <li class="menu-title pt-2 pb-1 text-xs">{{ t('noteList.exportFormat') }}</li>
        <li><a @click="exportContextNote('markdown')" class="flex items-center gap-1" :class="{'pointer-events-none opacity-50': loading}">
          <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 21h10a2 2 0 002-2V9.414a1 1 0 00-.293-.707l-5.414-5.414A1 1 0 0012.586 3H7a2 2 0 00-2 2v14a2 2 0 002 2z" />
          </svg>
          <span v-if="!loading">{{ t('noteList.markdownExport') }}</span>
          <span v-else class="flex items-center">
            <span class="loading loading-spinner loading-xs mr-1"></span>
            {{ t('noteList.exporting') }}
          </span>
        </a></li>
        <li><a @click="exportContextNote('pdf')" class="flex items-center gap-1" :class="{'pointer-events-none opacity-50': loading}">
          <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 21h10a2 2 0 002-2V9.414a1 1 0 00-.293-.707l-5.414-5.414A1 1 0 0012.586 3H7a2 2 0 00-2 2v14a2 2 0 002 2z" />
          </svg>
          <span v-if="!loading">{{ t('noteList.pdfExport') }}</span>
          <span v-else class="flex items-center">
            <span class="loading loading-spinner loading-xs mr-1"></span>
            {{ t('noteList.exporting') }}
          </span>
        </a></li>
        <li><a @click="exportContextNote('html')" class="flex items-center gap-1" :class="{'pointer-events-none opacity-50': loading}">
          <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 21h10a2 2 0 002-2V9.414a1 1 0 00-.293-.707l-5.414-5.414A1 1 0 0012.586 3H7a2 2 0 00-2 2v14a2 2 0 002 2z" />
          </svg>
          <span v-if="!loading">{{ t('noteList.htmlExport') }}</span>
          <span v-else class="flex items-center">
            <span class="loading loading-spinner loading-xs mr-1"></span>
            {{ t('noteList.exporting') }}
          </span>
        </a></li>
      </ul>
    </div>
    </Teleport>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch, onMounted, nextTick, onBeforeUnmount, onActivated } from 'vue'
import { useTipsStore, TipSummary } from '../stores/tipsStore'
import { useEncryptionStore } from '../stores/encryptionStore'
import { storeToRefs } from 'pinia'
import { useVirtualizer } from '@tanstack/vue-virtual'
import { useResponsive } from '../composables/useResponsive'
import { useI18n } from 'vue-i18n'

const { t, locale } = useI18n()

// 类型定义
interface Note extends TipSummary {
  isPinned?: boolean
}

// 组件属性
const props = defineProps({
  selectedNoteId: {
    type: String,
    default: ''
  },
  notebooks: {
    type: Array as () => { id: string, name: string, children?: any[] }[],
    default: () => []
  },
  selectedTags: {
    type: Array as () => string[],
    default: () => []
  },
  selectedNotebookId: {
    type: String,
    default: ''
  },
  // 新增：从外部传入的笔记数据
  tips: {
    type: Array as () => TipSummary[],
    default: () => []
  },
  // 新增：总笔记数量
  totalCount: {
    type: Number,
    default: 0
  }
})

// 组件事件
const emit = defineEmits(['select-note', 'search', 'new-note', 'delete-note', 'export-note', 'move-to-category', 'refresh', 'encrypt-note', 'decrypt-note'])

// Store
const tipsStore = useTipsStore()
const { isLoading: loading, hasMore } = storeToRefs(tipsStore)

// 数据库状态管理
import { useDatabaseStore } from '../stores/databaseStore'
const databaseStore = useDatabaseStore()
const { databaseChangeCounter } = storeToRefs(databaseStore)

// 加密store
const encryptionStore = useEncryptionStore()

// 响应式
const { isMobile } = useResponsive()

// 状态
const searchQuery = ref('')
const sortField = ref('updated')
const sortOrder = ref('desc')
const noteListContainer = ref<HTMLElement | null>(null)

// 右键菜单状态
const showContextMenu = ref(false)
const contextMenuX = ref(0)
const contextMenuY = ref(0)
const contextNote = ref<Note | null>(null)

// --- 多级菜单延迟关闭逻辑 ---
const openDropdowns = ref<string[]>([])
let closeTimeout: ReturnType<typeof setTimeout> | null = null

function handleDropdownEnter(id: string) {
  if (closeTimeout) {
    clearTimeout(closeTimeout)
    closeTimeout = null
  }
  if (!openDropdowns.value.includes(id)) {
    openDropdowns.value.push(id)
  }
}

function handleDropdownLeave(id: string) {
  closeTimeout = setTimeout(() => {
    openDropdowns.value = openDropdowns.value.filter(openId => openId !== id)
  }, 200)
}

onBeforeUnmount(() => {
  if (closeTimeout) clearTimeout(closeTimeout)
  // 清理右键菜单相关的事件监听器
  if (showContextMenu.value) {
    closeContextMenu()
  }
  document.removeEventListener('keydown', handleKeyDown)
  document.removeEventListener('mousedown', () => {}, true) // 清理可能残留的监听器
})

const filteredNotes = computed(() => {
  // 首先去重，避免重复的笔记ID
  const uniqueTips = new Map<string, TipSummary>()
  for (const tip of props.tips) {
    if (tip.id && !uniqueTips.has(tip.id)) {
      uniqueTips.set(tip.id, tip)
    }
  }

  let result: Note[] = Array.from(uniqueTips.values()).map((tip) => ({
    ...tip,
    isPinned: false, // You might want to handle pinning state properly
  }))

  // 本地搜索过滤（列表内搜索）
  if (searchQuery.value) {
    const query = searchQuery.value.toLowerCase()
    result = result.filter((note) => {
      return (note.title || '').toLowerCase().includes(query)
    })
  }

  // 排序
  result.sort((a, b) => {
    let aValue: string | number, bValue: string | number

    if (sortField.value === 'updated') {
      aValue = a.updated_at || 0
      bValue = b.updated_at || 0
    } else if (sortField.value === 'created') {
      aValue = a.created_at || 0
      bValue = b.created_at || 0
    } else {
      aValue = a.title || ''
      bValue = b.title || ''
    }

    // 字符串比较
    if (typeof aValue === 'string' && typeof bValue === 'string') {
      return sortOrder.value === 'asc'
        ? aValue.localeCompare(bValue)
        : bValue.localeCompare(aValue)
    }

    // 数字比较 (确保aValue和bValue都是数字)
    const aNum = typeof aValue === 'number' ? aValue : 0
    const bNum = typeof bValue === 'number' ? bValue : 0
    return sortOrder.value === 'asc' ? aNum - bNum : bNum - aNum
  })

  // 置顶固定的笔记
  result = [
    ...result.filter((note) => note.isPinned),
    ...result.filter((note) => !note.isPinned),
  ]

  return result
})

// 监听数据库切换事件，触发列表刷新
watch(databaseChangeCounter, (newCount, oldCount) => {
  if (newCount > oldCount && newCount > 0) {
    console.log('[NoteList] 检测到数据库切换/同步事件，触发数据刷新')
    // 触发父组件的refresh事件
    emit('refresh')
  }
})

// 监听笔记列表变化，确保虚拟列表正确更新
watch(() => filteredNotes.value.length, (newLength, oldLength) => {
  if (newLength !== oldLength && virtualizer.value) {
    nextTick(() => {
      try {
        // 重新计算虚拟列表
        virtualizer.value.measure()
      } catch (error) {
        console.error('Failed to update virtualizer after notes list change:', error)
      }
    })
  }
})

// 计算是否还有更多笔记
const hasMoreTips = computed(() => {
  // 如果选中了分类，检查分类浏览数据
  if (props.selectedNotebookId && tipsStore.currentCategoryBrowse) {
    const categoryData = tipsStore.currentCategoryBrowse
    const currentTipsCount = (categoryData.featured_tip ? 1 : 0) + categoryData.tip_summaries.length
    return currentTipsCount < categoryData.total_tips_count
  }
  
  // 否则使用常规的hasMore
  return hasMore.value
})

// 虚拟化
const virtualizer = useVirtualizer(
  computed(() => ({
    count: filteredNotes.value.length,
    getScrollElement: () => noteListContainer.value,
    estimateSize: () => 75, // 更接近实际高度的预估值
    overscan: 5,
  }))
)

const virtualItems = computed(() => virtualizer.value.getVirtualItems())

const totalSize = computed(() => virtualizer.value.getTotalSize())

// 方法
async function selectNote(note: Note) {
  if (note.is_encrypted) {
    const isUnlocked = encryptionStore.isItemUnlocked(note.id);
    if (!isUnlocked) {
      // 如果笔记已加密且未解锁，可以触发解锁流程
      // 这里暂时只传递ID，具体解锁逻辑在父组件处理
      emit('select-note', { id: note.id, content: t('noteList.encryptedPreview') });
      return;
    }
  }

  // 获取完整内容
  const content = await tipsStore.fetchTipContent(note.id);
  
  if (content !== null) {
    emit('select-note', { ...note, content });
  } else {
    // 处理获取内容失败的情况
    emit('select-note', { ...note, content: t('noteList.loadContentError') });
  }
}

function sortBy(field: string, order: 'asc' | 'desc') {
  sortField.value = field
  sortOrder.value = order
}

// 打开上下文菜单
async function openContextMenu(event: MouseEvent, note: Note) {
  // 由于已全局禁用默认右键菜单，这里不再需要preventDefault和stopPropagation
  
  // 立即设置上下文笔记，用于右键菜单操作
  contextNote.value = note
  
  // 使用页面坐标而不是客户端坐标
  let posX = event.pageX
  let posY = event.pageY
  
  // 立即设置初始位置
  contextMenuX.value = posX
  contextMenuY.value = posY
  
  // 先异步选中被右键点击的笔记，确保内容已加载
  try {
    // 如果笔记已加密且未解锁，使用特殊处理
    if (note.is_encrypted && !encryptionStore.isItemUnlocked(note.id)) {
      emit('select-note', { id: note.id, content: t('noteList.encryptedPreview') });
    } else {
      // 获取完整内容
      const content = await tipsStore.fetchTipContent(note.id);
      
      if (content !== null) {
        emit('select-note', { ...note, content });
      } else {
        // 处理获取内容失败的情况
        emit('select-note', { ...note, content: t('noteList.loadContentError') });
      }
    }
  } catch (error) {
    console.error('Failed to load note content for context menu:', error);
    // 确保即使加载内容失败，也能安全地发送选择事件
    emit('select-note', { ...note, content: t('noteList.loadContentError') });
  }
  
  // 使用nextTick确保在DOM更新后再显示菜单
  nextTick(() => {
    showContextMenu.value = true
    
    // 添加键盘事件监听
    document.addEventListener('keydown', handleKeyDown)
    
    // 确保菜单不会超出窗口边界
    setTimeout(() => {
      const menu = document.querySelector('.context-menu') as HTMLElement
      if (menu) {
        const menuWidth = menu.offsetWidth
        const menuHeight = menu.offsetHeight
        const windowWidth = window.innerWidth
        const windowHeight = window.innerHeight
        
        // 检查右边界
        if (posX + menuWidth > windowWidth) {
          posX = windowWidth - menuWidth - 10
        }
        
        // 检查下边界
        if (posY + menuHeight > windowHeight) {
          posY = windowHeight - menuHeight - 10
        }
        
        // 设置位置
        contextMenuX.value = posX
        contextMenuY.value = posY
        
        // 处理二级菜单位置
        const subMenus = menu.querySelectorAll('.dropdown-content')
        subMenus.forEach((subMenu: Element) => {
          const subMenuElement = subMenu as HTMLElement
          const subMenuRect = subMenuElement.getBoundingClientRect()
          
          // 检查二级菜单是否会超出右边界
          if (posX + menuWidth + subMenuRect.width > windowWidth) {
            // 如果会超出，则显示在左侧
            subMenuElement.style.left = 'auto'
            subMenuElement.style.right = '100%'
          }
        })
      }
    }, 0)

    // 点击其他区域关闭菜单
    const closeOnClickOutside = (e: MouseEvent) => {
      // 阻止点击事件导致的列表刷新
      e.stopPropagation()
      
      // 如果点击的不是菜单内部的元素，则关闭菜单
      const menu = document.querySelector('.context-menu')
      if (menu && !menu.contains(e.target as Node)) {
        closeContextMenu()
        document.removeEventListener('mousedown', closeOnClickOutside, true)
      }
    }
    
    // 使用捕获阶段监听点击事件，并添加阻止冒泡的逻辑
    document.addEventListener('mousedown', closeOnClickOutside, true)
  })
}

// 删除笔记
async function deleteContextNote() {
  if (!contextNote.value) {
    console.error('Error: No context note found for deletion')
    closeContextMenu()
    return
  }

  const noteToDelete = contextNote.value
  closeContextMenu()
  
  // 在发送删除事件前，先确保虚拟列表不再引用该笔记
  // 找到要删除笔记的索引
  const deleteIndex = filteredNotes.value.findIndex(note => note.id === noteToDelete.id)
  
  // 发送删除事件
  emit('delete-note', noteToDelete.id)
  
  // 如果找到了索引，手动更新virtualizer以防止引用错误
  if (deleteIndex !== -1 && virtualizer.value) {
    // 给virtualizer一点时间来处理DOM更新
    setTimeout(() => {
      try {
        // 重新计算虚拟列表
        virtualizer.value.measure()
      } catch (error) {
        console.error('Failed to update virtualizer after note deletion:', error)
      }
    }, 0)
  }
}

// 导出笔记
function exportContextNote(format: string) {
  if (!contextNote.value) {
    console.error('Error: No context note found for export')
    closeContextMenu()
    return
  }

  const noteToExport = contextNote.value
  console.log('准备导出笔记:', {
    noteId: noteToExport.id,
    noteTitle: noteToExport.title,
    format: format,
    selectedNoteId: props.selectedNoteId,
    isSelectedNote: noteToExport.id === props.selectedNoteId
  })

  emit('export-note', noteToExport.id, format)
  console.log('已发送导出事件到父组件:', noteToExport.id, format)
  closeContextMenu()
}

function getPreviewContent(note: Note): string {
  if (note.is_encrypted) {
    return t('noteList.encryptedPreview')
  }
  
  // 对于未加密的笔记，我们现在没有内容，可以显示摘要或通用提示
  // 这里可以根据需要从后端获取摘要
  return t('noteList.clickToView')
}

function formatDate(dateString: number): string {
  const date = new Date(dateString)
  const now = new Date()
  const diffMs = now.getTime() - date.getTime()
  const diffDays = Math.floor(diffMs / (1000 * 60 * 60 * 24))
  const currentLocale = locale.value || 'zh-CN'
  
  if (diffDays === 0) {
    // 今天
    return date.toLocaleTimeString(currentLocale, { hour: '2-digit', minute: '2-digit' })
  } else if (diffDays === 1) {
    // 昨天
    return t('noteList.yesterday')
  } else if (diffDays < 7) {
    // 本周
    return new Intl.DateTimeFormat(currentLocale, { weekday: 'long' }).format(date)
  } else {
    // 超过一周
    return date.toLocaleDateString(currentLocale, { month: 'numeric', day: 'numeric' })
  }
}

// 移动笔记到分类
function moveNoteToCategory(categoryId: string) {
  if (!contextNote.value) {
    console.error('错误：没有找到要移动的笔记上下文')
    closeContextMenu()
    return
  }

  const noteToMove = contextNote.value
  console.log('准备移动笔记到分类:', {
    noteId: noteToMove.id,
    noteTitle: noteToMove.title,
    targetCategoryId: categoryId,
    selectedNoteId: props.selectedNoteId,
    isSelectedNote: noteToMove.id === props.selectedNoteId
  })

  emit('move-to-category', noteToMove.id, categoryId)
  console.log('已发送移动事件到父组件:', noteToMove.id, categoryId)
  closeContextMenu()
}

// 加密笔记
function encryptContextNote() {
  if (!contextNote.value) {
    console.error('错误：没有找到要加密的笔记上下文')
    closeContextMenu()
    return
  }

  const noteToEncrypt = contextNote.value
  emit('encrypt-note', noteToEncrypt.id)
  closeContextMenu()
}

// 解密笔记
function decryptContextNote() {
  if (!contextNote.value) {
    console.error('错误：没有找到要解密的笔记上下文')
    closeContextMenu()
    return
  }

  const noteToDecrypt = contextNote.value
  emit('decrypt-note', noteToDecrypt.id)
  closeContextMenu()
}

// 统一的菜单关闭函数
function closeContextMenu() {
  showContextMenu.value = false
  contextNote.value = null // 清除上下文笔记
  // 移除键盘事件监听
  document.removeEventListener('keydown', handleKeyDown)
}

// 键盘事件处理
function handleKeyDown(e: KeyboardEvent) {
  if (e.key === 'Escape' && showContextMenu.value) {
    e.preventDefault()
    closeContextMenu()
  }
}

// 滚动加载
const handleScroll = () => {
  const container = noteListContainer.value
  if (container) {
    const { scrollTop, scrollHeight, clientHeight } = container
    if (scrollTop + clientHeight >= scrollHeight - 200 && !loading.value) {
      // 如果选中了分类，使用分类的加载更多
      if (props.selectedNotebookId && tipsStore.currentCategoryBrowse) {
        const categoryData = tipsStore.currentCategoryBrowse
        const currentTipsCount = (categoryData.featured_tip ? 1 : 0) + categoryData.tip_summaries.length
        if (currentTipsCount < categoryData.total_tips_count) {
          tipsStore.loadMoreCategoryTips(props.selectedNotebookId)
        }
      } else if (hasMore.value) {
        // 否则使用常规的分页加载
        tipsStore.fetchTips()
      }
    }
  }
}

// onMounted
onMounted(() => {
  tipsStore.fetchTips(true) // 初始加载第一页
})

// 添加onActivated钩子
onActivated(() => {
  // 仅确保选中的笔记项可见，不重新加载数据
  if (props.selectedNoteId && virtualizer.value) {
    const selectedIndex = filteredNotes.value.findIndex(note => note.id === props.selectedNoteId)
    if (selectedIndex !== -1) {
      virtualizer.value.scrollToIndex(selectedIndex, { align: 'center' })
    }
  }
})

// 检查笔记是否加密
function isNoteEncrypted(noteId: string): boolean {
  return encryptionStore.isItemEncrypted(noteId)
}

</script>

<style scoped>
/* 由于虚拟化与TransitionGroup不兼容，移除相关动画样式 */
/* .note-list-enter-active,
.note-list-leave-active {
  transition: all 0.4s cubic-bezier(0.2, 0.8, 0.2, 1.0);
  position: relative;
}
.note-list-enter-from {
  opacity: 0;
  transform: translateY(30px);
}
.note-list-leave-to {
  opacity: 0;
  transform: translateX(-30px);
  position: absolute;
  width: 100%;
}
.note-list-move {
  transition: transform 0.5s ease;
} */

/* 卡片项动画 */
.note-card-enter-active,
.note-card-leave-active {
  transition: all 0.4s cubic-bezier(0.2, 0.8, 0.2, 1.0);
  position: relative;
}
.note-card-enter-from {
  opacity: 0;
  transform: scale(0.9) translateY(20px);
}
.note-card-leave-to {
  opacity: 0;
  transform: scale(0.8);
  position: absolute;
}
.note-card-move {
  transition: transform 0.5s ease;
}

/* 防止布局抖动 */
.note-list-item, .note-card-item {
  backface-visibility: hidden;
  will-change: opacity, transform;
  padding-left: 8px; /* 增加一些左边距 */
  padding-right: 8px; /* 增加一些右边距 */
  box-sizing: border-box; /* 确保 padding 不会影响宽度 */
}

/* 多级分类菜单 hover 展示子菜单 */
.dropdown-content {
  display: none;
  min-width: 180px;
}
.dropdown.dropdown-hover:hover > .dropdown-content,
.dropdown.dropdown-hover:focus-within > .dropdown-content {
  display: block;
}

/* 保证子菜单定位在右侧 */
.dropdown-content {
  position: absolute;
  left: calc(100% - 12px);
  top: 0;
  z-index: 10;
}

/* 菜单项不换行 */
.menu .whitespace-nowrap {
  white-space: nowrap;
}

/* 优化有子菜单项的可交互区域 */
.dropdown.dropdown-hover > .dropdown-trigger {
  display: flex;
  align-items: center;
  width: 100%;
  min-width: 180px;
  padding: 8px 12px;
  cursor: pointer;
  border-radius: 0.5rem;
  transition: background 0.2s;
}
.dropdown.dropdown-hover > .dropdown-trigger:hover,
.dropdown.dropdown-hover:focus-within > .dropdown-trigger {
  background: var(--color-base-200, #f3f4f6);
}

/* 让箭头和文字都在可交互区域内 */
.dropdown-trigger .arrow {
  margin-left: auto;
  pointer-events: none;
}

/* 在多级菜单li上加事件绑定和动态class */
.dropdown-open > .dropdown-content {
  display: block !important;
}

/* .note-list-leave-active {
  transition: all 0.3s cubic-bezier(.55,0,.1,1);
  position: relative;
  z-index: 1;
}
.note-list-leave-to {
  opacity: 0;
  transform: translateY(-20px) scaleY(0.8);
  height: 0 !important;
  margin: 0 !important;
  padding: 0 !important;
}
.note-list-move {
  transition: transform 0.3s cubic-bezier(.55,0,.1,1);
} */

/* NoteList特有的右键菜单样式 */
.context-menu {
  min-width: 200px;
  max-width: 300px;
  backdrop-filter: blur(8px);
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.1);
}

/* 笔记预览文本的特殊截断样式 */
.note-preview-text {
  display: -webkit-box;
  -webkit-box-orient: vertical;
  overflow: hidden;
  line-height: 1.4;
}

/* 笔记列表项的特殊悬停效果 */
.note-list-item:hover {
  transform: translateY(-1px);
  transition: transform 0.2s ease;
}

/* 固定笔记的特殊指示器 */
.pin-indicator {
  position: absolute;
  top: 8px;
  right: 8px;
  opacity: 0.7;
  transition: opacity 0.2s ease;
}

.note-list-item:hover .pin-indicator {
  opacity: 1;
}
</style> 