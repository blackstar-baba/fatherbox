<script lang="ts" setup>
import { onMounted, onUnmounted, ref } from 'vue';

import { Page } from '@vben/common-ui';

import { CopyOutlined, DoubleRightOutlined } from '@ant-design/icons-vue';
import { message, type SelectProps } from 'ant-design-vue';
import {
  Button,
  Card,
  Col,
  Divider,
  Input,
  Row,
  Select,
  Switch,
  Tooltip,
} from 'ant-design-vue';

import { copy } from '#/utils';
import { formatToDateTime } from '#/utils/dateUtil';

const formatter = 'YYYY-MM-DD HH:mm:ss';
const fullFormatter = 'YYYY-MM-DD HH:mm:ss.SSS';

const checked = ref<boolean>(true);
const nowDate = new Date();
const txtNowS = ref(Math.round(nowDate.getTime() / 1000));
const txtNowMs = ref(nowDate.getTime());
const txtNowDate = ref(formatToDateTime(nowDate, formatter));
const txtSrcStamp = ref(txtNowS.value);
const txtDesDate = ref(txtNowDate.value);
const txtSrcDate = ref(txtNowDate.value);
const txtDesStamp = ref(txtNowS.value);
const secFrom = ref('s');
const secTo = ref('s');
// let worldTime = {};
const curUTC = ref(`${(new Date().getTimezoneOffset() / 60) * -1}`);
let intervalId = -1;
const options1 = ref<SelectProps['options']>([
  {
    label: 'Eniwetok (UTC -12)',
    value: '-12',
  },
  {
    label: 'Samoa (UTC -11)',
    value: '-11',
  },
  {
    label: 'Hawaii State (UTC -10)',
    value: '-10',
  },
  {
    label: '(UTC -9)',
    value: '-9',
  },
  {
    label: '(UTC -8)',
    value: '-8',
  },
  {
    label: '(UTC -7)',
    value: '-7',
  },
  {
    label: '(UTC -6)',
    value: '-6',
  },
  {
    label: 'NewYork (UTC -5)',
    value: '-5',
  },
  {
    label: '(UTC -4)',
    value: '-4',
  },
  {
    label: 'Brazil (UTC -3)',
    value: '-3',
  },
  {
    label: '(UTC -2)',
    value: '-2',
  },
  {
    label: '(UTC -1)',
    value: '-1',
  },
  {
    label: 'London（UTC）',
    value: '0',
  },
  {
    label: 'Roma (UTC +1)',
    value: '1',
  },
  {
    label: 'Israel (UTC +2)',
    value: '2',
  },
  {
    label: 'Moscow (UTC +3)',
    value: '3',
  },
  {
    label: 'Baku (UTC +4)',
    value: '4',
  },
  {
    label: 'New Delhi (UTC +5)',
    value: '5',
  },
  {
    label: 'Dakar (UTC +6)',
    value: '6',
  },
  {
    label: 'Bangkok (UTC +7)',
    value: '7',
  },
  {
    label: 'Beijing (UTC +8)',
    value: '8',
  },
  {
    label: 'ToKio (UTC +9)',
    value: '9',
  },
  {
    label: 'Sydney (UTC +10)',
    value: '10',
  },
  {
    label: '(UTC +11)',
    value: '11',
  },
  {
    label: 'Wellington (UTC +12)',
    value: '12',
  },
]);

const options2 = ref<SelectProps['options']>([
  {
    label: 's',
    value: 's',
  },
  {
    label: 'ms',
    value: 'ms',
  },
]);

onMounted(() => {
  startTimestamp();
});

onUnmounted(() => {
  stopTimeStamp();
});

function switchAuthRefresh() {
  if (checked.value === true) {
    startTimestamp();
  } else {
    stopTimeStamp();
  }
}

function startTimestamp() {
  intervalId = window.setInterval(() => {
    const localDate = new Date();
    const gmtTime = new Date(
      localDate.getTime() + localDate.getTimezoneOffset() * 60_000,
    );

    const nowDate = new Date(
      gmtTime.getTime() + Number(curUTC.value) * 60 * 60_000,
    );
    txtNowDate.value = formatToDateTime(nowDate, formatter);

    txtNowS.value = Math.round(nowDate.getTime() / 1000);
    txtNowMs.value = nowDate.getTime();

    // worldTime['local'] = txtNowDate.value;
    // worldTime['utc'] = txtNowDate.value;
    //
    // for (let offset = -12; offset <= 12; offset++) {
    //   let date = new Date(gmtTime.getTime() + offset * 60 * 60000);
    //   worldTime[offset > 0 ? '+' + offset : offset] = formatToDateTime(date, formatter);
    // }
  }, 1000);
}

function stopTimeStamp() {
  if (intervalId !== -1) {
    clearInterval(intervalId);
  }
}
function stampToLocale() {
  if (txtSrcStamp.value === 0) {
    message.error('Can not find Unix Timestamp');
    return;
  }
  if (!Number.parseInt(txtSrcStamp.value.toString(), 10)) {
    message.error('Incorrect Unix Timestamp');
    return;
  }

  const base = secFrom.value === 's' ? 1000 : 1;
  const date = new Date(
    Number.parseInt(txtSrcStamp.value.toString(), 10) * base +
      (new Date().getTimezoneOffset() + Number(curUTC.value) * 60) * 60_000,
  );
  const format = secFrom.value === 's' ? formatter : fullFormatter;
  txtDesDate.value = formatToDateTime(date, format);
}

function localeToStamp() {
  if (txtSrcDate.value && !/\s\d+:\d+:\d+/.test(txtSrcDate.value)) {
    txtSrcDate.value += ' 00:00:00';
  }
  const locale = new Date(
    Date.parse(txtSrcDate.value) -
      (new Date().getTimezoneOffset() + Number(curUTC.value) * 60) * 60_000,
  ).getTime();
  if (Number.isNaN(locale)) {
    message.error('Incorrect Time，e.g. 2014-04-01 10:01:01 or 2014-01-01');
  }
  const base = secTo.value === 's' ? 1000 : 1;
  txtDesStamp.value = Math.round(locale / base);
}
</script>
<template>
  <Page
    description="TimeStamp to DataTime & DateTime to TimeStamp "
    title="Time Parser"
  >
    <Row :gutter="16">
      <Col :md="24">
        <div class="py-4">
          <span class="m-4 leading-8">TimeZone:</span>
          <Select
            v-model:value="curUTC"
            :options="options1"
            class="ml-2 w-60"
          />
        </div>
      </Col>
      <Col :md="12">
        <Card :bordered="false">
          <Divider orientation="left">
            Current
            <Switch
              v-model:checked="checked"
              checked-children="Auto Refresh"
              class="m-2"
              un-checked-children="Off"
              @click="switchAuthRefresh"
            />
          </Divider>
          <Row :gutter="16">
            <Col :md="6">
              <span class="leading-8">DateTime:</span>
            </Col>
            <Col :md="18">
              <Input
                v-model:value="txtNowDate"
                :bordered="false"
                class="w-52"
                readonly
              />
              <Button @click="copy(txtNowDate)">
                <template #icon><CopyOutlined /></template>
              </Button>
            </Col>
          </Row>
          <Row :gutter="16">
            <Col :md="6">
              <span class="leading-8">TimeStamp(s):</span>
            </Col>
            <Col :md="18">
              <Input
                v-model:value="txtNowS"
                :bordered="false"
                class="w-52"
                readonly
              />
              <Button @click="copy(txtNowS)">
                <template #icon><CopyOutlined /></template>
              </Button>
            </Col>
          </Row>
          <Row :gutter="16">
            <Col :md="6">
              <span class="leading-8">TimeStamp(ms):</span>
            </Col>
            <Col :md="18">
              <Input
                v-model:value="txtNowMs"
                :bordered="false"
                class="w-52"
                readonly
              />
              <Button @click="copy(txtNowMs)">
                <template #icon><CopyOutlined /></template>
              </Button>
            </Col>
          </Row>
        </Card>
      </Col>
      <Col :md="12">
        <Card :bordered="false">
          <Divider orientation="left">Convert</Divider>
          <Row :gutter="16">
            <Col :md="24">
              <span class="leading-12">TimeStamp to DateTime</span>
            </Col>
            <Col :md="24">
              <Input
                v-model:value="txtSrcStamp"
                class="w-52"
                placeholder="1388307215"
              />
              <Select
                v-model:value="secFrom"
                :options="options2"
                class="m-2 w-16"
              />
              <Tooltip title="convert to">
                <Button type="text" @click="stampToLocale()">
                  <template #icon><DoubleRightOutlined /></template>
                </Button>
              </Tooltip>
              {{ txtDesDate }}
            </Col>
          </Row>
          <Row :gutter="16">
            <Col :md="24">
              <span class="leading-12">DateTime to TimeStamp</span>
            </Col>
            <Col :md="24">
              <Input
                v-model:value="txtSrcDate"
                class="w-52"
                placeholder="2015-04-01 10:01:01"
              />
              <Button type="text" @click="localeToStamp()">
                <template #icon><DoubleRightOutlined /></template>
              </Button>
              {{ txtDesStamp }}
              <Select
                v-model:value="secTo"
                :options="options2"
                class="m-2 w-16"
              />
            </Col>
          </Row>
        </Card>
      </Col>
    </Row>
  </Page>
</template>
