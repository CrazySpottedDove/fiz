<script setup>
import { reactive, onMounted, computed } from "vue";
import VuePdfEmbed from "vue-pdf-embed";
import { createLoadingTask } from "vue3-pdfjs";
const props = defineProps({
    pdfUrl: String
})

const state = reactive({
    source: props.pdfUrl, // 预览pdf文件地址
    pageNum: 1, // 当前页面
    scale: 10, // 缩放比例（初始值会在 onMounted 中根据 PDF 大小调整）
    numPages: 0, // 总页数
});

onMounted(async () => {
    const loadingTask = createLoadingTask(state.source);
    const pdf = await loadingTask.promise;
    state.numPages = pdf.numPages;
    // 获取第一页尺寸，初始 scale 设置成让 PDF 不超过视口 80%
    const page = await pdf.getPage(1);
    const viewport = page.getViewport({ scale: 1 });
    // 容器目标大小（80% 的窗口宽高）
    const containerWidth = document.body.clientWidth * 0.8;
    const containerHeight = document.body.clientHeight * 0.8;
    const scaleWidth = containerWidth / viewport.width;
    const scaleHeight = containerHeight / viewport.height;
    const initialScale = Math.min(scaleWidth, scaleHeight);
    state.scale = initialScale;
    console.log(initialScale)
});

const scale = computed(() => `transform:scale(${state.scale})`);

function lastPage() {
    if (state.pageNum > 1) {
        state.pageNum -= 1;
    }
}
function nextPage() {
    if (state.pageNum < state.numPages) {
        state.pageNum += 1;
    }
}
function pageZoomOut() {
    if (state.scale < 2) {
        state.scale += 0.1;
    }
}
function pageZoomIn() {
    if (state.scale > 1) {
        state.scale -= 0.1;
    }
}
</script>

<template>
    <!-- 外层容器保持 80vh 高度，超出部分滚动 -->
    <div class="flex justify-center h-[80vh] w-full overflow-y-auto">
        <vue-pdf-embed :source="state.source" :style="scale" class="vue-pdf-embed" :page="state.pageNum" />
    </div>
    <div class="page-tool">
        <div class="page-tool-item" @click="lastPage">上一页</div>
        <div class="page-tool-item" @click="nextPage">下一页</div>
        <div class="page-tool-item">{{ state.pageNum }}/{{ state.numPages }}</div>
        <div class="page-tool-item" @click="pageZoomOut">放大</div>
        <div class="page-tool-item" @click="pageZoomIn">缩小</div>
    </div>
</template>

<style lang="css" scoped>
/* 根据 PDF 本身的长宽比，初始比例通过 JS 计算，
   CSS 限制最大宽高为 80% 可保证预览正常显示 */
.vue-pdf-embed {
    text-align: center;
    max-width: 80%;
    max-height: 80%;
    border: 1px solid #e5e5e5;
    margin: 0 auto;
    box-sizing: border-box;
}

.page-tool {
    position: absolute;
    bottom: 35px;
    padding-left: 15px;
    padding-right: 15px;
    display: flex;
    align-items: center;
    background: rgb(66, 66, 66);
    color: white;
    border-radius: 19px;
    z-index: 100;
    cursor: pointer;
    left: 50%;
    transform: translateX(-50%);
}

.page-tool-item {
    padding: 8px 15px;
    cursor: pointer;
}
</style>