<template>
  <PageWrapper
    contentBackground
    title="Time Parser"
    content="TimeStamp to DataTime & DateTime to TimeStamp "
  >
    <Row :gutter="16">
      <Col :md="24">
        <Row class="p-4">
          <Col :md="3" class="leading-8"> TimeZone:</Col>
          <Col :md="21">
            <Select ref="select" v-model:value="curUTC" class="w-100" :options="options1" />
            <Switch
              class="m-2"
              v-model:checked="checked"
              checked-children="Auto"
              un-checked-children="Off"
              @click="switchAuthRefresh"
            />
          </Col>
        </Row>
      </Col>
      <Col :md="10">
        <Row class="p-4">
          <Col :md="6" class="leading-8">DateTime:</Col>
          <Col :md="18">
            <Input v-model:value="txtNowDate" class="w-50" readonly :bordered="false" />
            <Button @click="copyDateToClipboard">
              <template #icon><CopyOutlined /></template>
            </Button>
          </Col>
        </Row>
        <Row class="p-4">
          <Col :md="6" class="leading-8">TimeStamp(s):</Col>
          <Col :md="18">
            <Input v-model:value="txtNowS" class="w-50" readonly :bordered="false" />
            <Button @click="copyTimeStampSToClipboard">
              <template #icon><CopyOutlined /></template>
            </Button>
          </Col>
        </Row>
        <Row class="p-4">
          <Col :md="6" class="leading-8">TimeStamp(ms):</Col>
          <Col :md="18">
            <Input v-model:value="txtNowMs" class="w-50" readonly :bordered="false" />
            <Button @click="copyTimeStampMsToClipboard">
              <template #icon><CopyOutlined /></template>
            </Button>
          </Col>
        </Row>
      </Col>
      <Col :md="13">
        <Row class="p-4">
          <Col :md="6" class="leading-12">TimeStamp to DateTime:</Col>
          <Col :md="18">
            <Input v-model:value="txtSrcStamp" placeholder="1388307215" class="m-2 w-45" />
            <Select ref="select" v-model:value="secFrom" class="m-2 w-20" :options="options2" />
            <Tooltip title="convert to">
              <Button type="text" @click="stampToLocale()">
                <template #icon><DoubleRightOutlined /></template>
              </Button>
            </Tooltip>
            {{ txtDesDate }}
          </Col>
        </Row>
        <Row class="p-4">
          <Col :md="6" class="leading-12"> DateTime to TimeStamp:</Col>
          <Col :md="18">
            <Input v-model:value="txtSrcDate" placeholder="2015-04-01 10:01:01" class="m-2 w-60" />
            <Select ref="select" v-model:value="secTo" class="m-2 w-20" :options="options2" />
            <Button type="text" @click="localeToStamp()">
              <template #icon><DoubleRightOutlined /></template>
            </Button>
            {{ txtDesStamp }}
          </Col>
        </Row>
      </Col>
    </Row>
  </PageWrapper>
</template>
<script lang="ts" setup>
  import { ref, onMounted, onUnmounted } from 'vue';
  import { formatToDateTime } from '@/utils/dateUtil';
  import { Select, Input, Button, Switch, Tooltip, Row, Col } from 'ant-design-vue';
  import { CopyOutlined, DoubleRightOutlined } from '@ant-design/icons-vue';
  import type { SelectProps } from 'ant-design-vue';
  import { PageWrapper } from '@/components/Page';

  const formatter = 'YYYY-MM-DD HH:mm:ss';
  const fullFormatter = 'YYYY-MM-DD HH:mm:ss.SSS';

  let checked = ref<boolean>(true);
  let nowDate = new Date();
  let txtNowS = ref(Math.round(nowDate.getTime() / 1000));
  let txtNowMs = ref(nowDate.getTime());
  let txtNowDate = ref(formatToDateTime(nowDate, formatter));
  let txtSrcStamp = ref(txtNowS.value);
  let txtDesDate = ref(txtNowDate.value);
  let txtSrcDate = ref(txtNowDate.value);
  let txtDesStamp = ref(txtNowS.value);
  let secFrom = ref('s');
  let secTo = ref('s');
  // let worldTime = {};
  let curUTC = ref((new Date().getTimezoneOffset() / 60) * -1 + '');
  let intervalId = -1;
  const options1 = ref<SelectProps['options']>([
    {
      value: '-12',
      label: 'Eniwetok (UTC -12)',
    },
    {
      value: '-11',
      label: 'Samoa (UTC -11)',
    },
    {
      value: '-10',
      label: 'Hawaii State (UTC -10)',
    },
    {
      value: '-9',
      label: '(UTC -9)',
    },
    {
      value: '-8',
      label: '(UTC -8)',
    },
    {
      value: '-7',
      label: '(UTC -7)',
    },
    {
      value: '-6',
      label: '(UTC -6)',
    },
    {
      value: '-5',
      label: 'NewYork (UTC -5)',
    },
    {
      value: '-4',
      label: '(UTC -4)',
    },
    {
      value: '-3',
      label: 'Brazil (UTC -3)',
    },
    {
      value: '-2',
      label: '(UTC -2)',
    },
    {
      value: '-1',
      label: '(UTC -1)',
    },
    {
      value: '0',
      label: 'London（UTC）',
    },
    {
      value: '1',
      label: 'Roma (UTC +1)',
    },
    {
      value: '2',
      label: 'Israel (UTC +2)',
    },
    {
      value: '3',
      label: 'Moscow (UTC +3)',
    },
    {
      value: '4',
      label: 'Baku (UTC +4)',
    },
    {
      value: '5',
      label: 'New Delhi (UTC +5)',
    },
    {
      value: '6',
      label: 'Dakar (UTC +6)',
    },
    {
      value: '7',
      label: 'Bangkok (UTC +7)',
    },
    {
      value: '8',
      label: 'Beijing (UTC +8)',
    },
    {
      value: '9',
      label: 'ToKio (UTC +9)',
    },
    {
      value: '10',
      label: 'Sydney (UTC +10)',
    },
    {
      value: '11',
      label: '(UTC +11)',
    },
    {
      value: '12',
      label: 'Wellington (UTC +12)',
    },
  ]);

  const options2 = ref<SelectProps['options']>([
    {
      value: 's',
      label: 's',
    },
    {
      value: 'ms',
      label: 'ms',
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
      let localDate = new Date();
      let gmtTime = new Date(localDate.getTime() + localDate.getTimezoneOffset() * 60000);

      let nowDate = new Date(gmtTime.getTime() + Number(curUTC.value) * 60 * 60000);
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
      alert('Can not find Unix Timestamp');
      return;
    }
    if (!parseInt(txtSrcStamp.value, 10)) {
      alert('Incorrect Unix Timestamp');
      return;
    }

    let base = secFrom.value === 's' ? 1000 : 1;
    let date = new Date(
      parseInt(txtSrcStamp.value, 10) * base +
        (new Date().getTimezoneOffset() + Number(curUTC.value) * 60) * 60000,
    );
    let format = secFrom.value == 's' ? formatter : fullFormatter;
    txtDesDate.value = formatToDateTime(date, format);
  }

  function localeToStamp() {
    if (txtSrcDate.value && !/\s\d+:\d+:\d+/.test(txtSrcDate.value)) {
      txtSrcDate.value += ' 00:00:00';
    }
    let locale = new Date(
      Date.parse(txtSrcDate.value) -
        (new Date().getTimezoneOffset() + Number(curUTC.value) * 60) * 60000,
    ).getTime();
    if (isNaN(locale)) {
      alert('Incorrect Time，e.g. 2014-04-01 10:01:01 or 2014-01-01');
    }
    let base = secTo.value === 's' ? 1000 : 1;
    txtDesStamp.value = Math.round(locale / base);
  }

  function copyDateToClipboard() {
    copy(txtNowDate.value);
  }

  function copyTimeStampSToClipboard() {
    copy(txtNowS.value);
  }

  function copyTimeStampMsToClipboard() {
    copy(txtNowMs.value);
  }

  function copy(value) {
    let input = document.createElement('textarea');
    input.style.position = 'fixed';
    input.style.opacity = '0';
    input.value = value;
    document.body.appendChild(input);
    input.select();
    document.execCommand('Copy');
    document.body.removeChild(input);
  }
</script>
